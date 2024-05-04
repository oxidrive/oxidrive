package file

import (
	"context"
	"database/sql"
	"errors"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ file.Files = (*PgFiles)(nil)

type PgFiles struct {
	db *sqlx.DB
}

func NewPgFiles(db *sqlx.DB) *PgFiles {
	return &PgFiles{db: db}
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

func (s *PgFiles) ByOwnerByPath(ctx context.Context, owner user.ID, path file.Path) (*file.File, error) {
	var f pgFile
	err := s.db.GetContext(ctx, &f, "select id, name, path, size, user_id from files where user_id = $1 and path = $2", owner.String(), path)
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
