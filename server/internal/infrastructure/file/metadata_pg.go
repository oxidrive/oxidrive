package file

import (
	"context"

	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
)

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
