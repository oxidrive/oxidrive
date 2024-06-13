package file

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"strconv"
	"strings"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ file.Files = (*SqliteFiles)(nil)

type SqliteFiles struct {
	db *sqlx.DB
}

func NewSqliteFiles(db *sqlx.DB) *SqliteFiles {
	return &SqliteFiles{db: db}
}

func (s *SqliteFiles) List(ctx context.Context, prefix *file.Path, params list.Params) (list.Of[file.File], error) {
	after := uint64(0)
	if params.After != nil {
		c := list.DecodeCursor(*params.After)
		a, err := strconv.ParseUint(c, 10, 64)
		if err != nil {
			return list.Empty[file.File](), fmt.Errorf("%s: %w", list.ErrInvalidAfter, err)
		}

		after = a
	}

	regex := ".*"
	if prefix != nil {
		regex = fmt.Sprintf("^%s/[^/]*$", strings.TrimSuffix(prefix.String(), "/"))
	}

	// We fetch the required amount of items plus one from the next slice to use as the Next cursor
	limit := params.First + 1

	var count int
	err := s.db.GetContext(ctx, &count, "select count(id) from files where path regexp $1", regex)
	if err != nil {
		return list.Empty[file.File](), err
	}

	if count == 0 {
		return list.Empty[file.File](), nil
	}

	var pff []sqliteFile
	err = s.db.SelectContext(ctx, &pff, `
with
    numbered_files as (
        select row_number() over (order by type desc, path) as cursor, * from files where path regexp $2 order by type desc, path
    )
select * from numbered_files where cursor >= $1 limit $3
`, after, regex, limit)
	if err != nil {
		return list.Empty[file.File](), err
	}

	if len(pff) == 0 {
		return list.Empty[file.File](), nil
	}

	items := make([]file.File, len(pff))
	var cursor uint64
	for i, pf := range pff {
		cursor = pf.Cursor
		items[i] = *pf.into()
	}

	var next *list.Cursor
	if len(items) == limit {
		// We remove the last one as it's not really part of the current slice, we just need its ID to  use as the Next cursor
		// If we fetched less than params.Limit + 1, it means we're at the end of the collection and there are no more pages after that
		idx := len(items) - 1
		c := list.EncodeCursor(strconv.FormatUint(cursor, 10))
		next = &c
		items = items[:idx]
	}

	return list.Of[file.File]{
		Items: items,
		Count: len(items),
		Total: count,
		Next:  next,
	}, nil
}

func (s *SqliteFiles) Save(ctx context.Context, f file.File) (*file.File, error) {
	if f.Type == file.TypeFolder {
		return nil, file.ErrFolderSave
	}

	tx, err := s.db.BeginTxx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to open DB transaction: %w", err)
	}

	if err := s.saveFolder(ctx, tx, &f); err != nil {
		if err := tx.Rollback(); err != nil {
			return nil, fmt.Errorf("failed to rollback DB transaction: %w", err)
		}
		return nil, fmt.Errorf("failed to save folder in DB: %w", err)
	}

	_, err = tx.ExecContext(ctx, `insert into files (
        id,
        type,
        content_type,
        name,
        path,
        size,
        user_id
    ) values (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7
    )
    on conflict (id)
    do update set
      content_type = excluded.content_type,
      name = excluded.name,
      path = excluded.path,
      size = excluded.size
    ;`, f.ID.String(), f.Type, f.ContentType, f.Name, f.Path, f.Size, f.OwnerID.String())
	if err != nil {
		if err := tx.Rollback(); err != nil {
			return nil, fmt.Errorf("failed to rollback DB transaction: %w", err)
		}

		return nil, fmt.Errorf("failed to save file in DB: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit DB transaction: %w", err)
	}
	return &f, nil
}

func (s *SqliteFiles) saveFolder(ctx context.Context, tx *sqlx.Tx, f *file.File) error {
	d := f.Folder()
	if d == nil {
		return nil
	}

	_, err := tx.ExecContext(ctx, `insert into files (
        id,
        type,
        content_type,
        name,
        path,
        size,
        user_id
    ) values (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7
    )
    on conflict (path)
    do update set
      type = excluded.type,
      size = files.size + excluded.size
    ;`, file.NewID().String(), file.TypeFolder, file.ContentTypeFolder, d.Name, d.Path, f.Size, f.OwnerID.String())

	return err
}

func (s *SqliteFiles) ByID(ctx context.Context, id file.ID) (*file.File, error) {
	var f sqliteFile
	err := s.db.GetContext(ctx, &f, "select id, type, content_type, name, path, size, user_id from files where id = $1", id.String())
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return f.into(), nil
}

func (s *SqliteFiles) ByOwnerByPath(ctx context.Context, owner user.ID, path file.Path) (*file.File, error) {
	var f sqliteFile
	err := s.db.GetContext(ctx, &f, "select id, type, content_type, name, path, size, user_id from files where user_id = $1 and path = $2", owner.String(), path)
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return f.into(), nil
}

type sqliteFile struct {
	Cursor      uint64    `db:"cursor"`
	ID          uuid.UUID `db:"id"`
	Type        string    `db:"type"`
	ContentType string    `db:"content_type"`
	Name        string    `db:"name"`
	Path        string    `db:"path"`
	Size        int       `db:"size"`
	UserID      uuid.UUID `db:"user_id"`
}

func (sf sqliteFile) into() *file.File {
	return &file.File{
		ID:          file.ID(sf.ID),
		Type:        file.Type(sf.Type),
		Content:     nil,
		ContentType: file.ContentType(sf.ContentType),
		Name:        file.Name(sf.Name),
		Path:        file.Path(sf.Path),
		Size:        file.Size(sf.Size),
		OwnerID:     user.ID(sf.UserID),
	}
}
