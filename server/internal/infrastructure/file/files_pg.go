package file

import (
	"context"
	"database/sql"
	"errors"
	"fmt"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ file.Files = (*PgFiles)(nil)

type PgFiles struct {
	db *sqlx.DB
}

func NewPgFiles(db *sqlx.DB) *PgFiles {
	return &PgFiles{db: db}
}

func (p *PgFiles) List(ctx context.Context, prefix *file.Path, params list.Params) (list.Of[file.File], error) {
	after := uuid.Nil
	if params.After != nil {
		a, err := uuid.Parse(*params.After)
		if err != nil {
			return list.Empty[file.File](), fmt.Errorf("%s: %w", list.ErrInvalidAfter, err)
		}

		after = a
	}

	regex := ".*"
	if prefix != nil {
		regex = fmt.Sprintf("^%s/[^/]*$", prefix.String())
	}

	// We fetch the required amount of items, plus one from the next slice to use as the Next cursor
	limit := params.First + 1

	var count int
	err := p.db.GetContext(ctx, &count, "select count(id) from files where path ~* $1", regex)
	if err != nil {
		return list.Empty[file.File](), err
	}

	if count == 0 {
		return list.Empty[file.File](), nil
	}

	var pff []pgFile
	err = p.db.SelectContext(ctx, &pff, "select id, name, path, size, user_id from files where id >= $1 and path ~* $2 order by id asc limit $3", after, regex, limit)
	if err != nil {
		return list.Empty[file.File](), err
	}

	if len(pff) == 0 {
		return list.Empty[file.File](), nil
	}

	items := make([]file.File, len(pff))
	for i, pf := range pff {
		items[i] = *pf.into()
	}

	var next *string
	if len(items) == limit {
		// We remove the last one as it's not really part of the current slice, we just need its ID to use as the Next cursor
		// If we fetched less than params.Limit + 1, it means we're at the end of the collection and there are no more pages after that
		idx := len(items) - 1
		n := items[idx].ID.String()
		next = &n
		items = items[:idx]
	}

	return list.Of[file.File]{
		Items: items,
		Count: len(items),
		Total: count,
		Next:  next,
	}, nil
}

func (p *PgFiles) Save(ctx context.Context, f file.File) (*file.File, error) {
	if _, err := p.db.ExecContext(ctx, `insert into files (
        id,
        name,
        path,
        size,
        user_id
    ) values (
        $1,
        $2,
        $3,
        $4,
        $5
    )
    on conflict (id)
    do update set
      name = excluded.name,
      path = excluded.path,
      size = excluded.size
    ;`, f.ID.String(), f.Name, f.Path, f.Size, f.OwnerID.String()); err != nil {
		return nil, err
	}

	return &f, nil
}

func (p *PgFiles) ByID(ctx context.Context, id file.ID) (*file.File, error) {
	var f pgFile
	err := p.db.GetContext(ctx, &f, "select id, name, path, size, user_id from files where id = $1", id.String())
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return f.into(), nil
}

func (p *PgFiles) ByOwnerByPath(ctx context.Context, owner user.ID, path file.Path) (*file.File, error) {
	var f pgFile
	err := p.db.GetContext(ctx, &f, "select id, name, path, size, user_id from files where user_id = $1 and path = $2", owner.String(), path)
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return f.into(), nil
}

type pgFile struct {
	ID     uuid.UUID `db:"id"`
	Name   string    `db:"name"`
	Path   string    `db:"path"`
	Size   int       `db:"size"`
	UserID uuid.UUID `db:"user_id"`
}

func (sf pgFile) into() *file.File {
	return &file.File{
		ID:      file.ID(sf.ID),
		Content: nil,
		Name:    file.Name(sf.Name),
		Path:    file.Path(sf.Path),
		Size:    file.Size(sf.Size),
		OwnerID: user.ID(sf.UserID),
	}
}
