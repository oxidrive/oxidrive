package file

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/rs/zerolog"
)

type PgFiles struct {
	db *sqlx.DB
}

func NewPgFiles(db *sqlx.DB) *PgFiles {
	return &PgFiles{db: db}
}

func (p *PgFiles) Save(ctx context.Context, f file.File, logger zerolog.Logger) (file.File, error) {
	_, err := p.db.ExecContext(ctx, `insert into files (
        id,
        name,
        path,
        size
    ) values (
        $1,
        $2,
        $3,
        $4
    )
    on conflict (id)
    do update set
      name = excluded.name,
      path = excluded.path,
      size = excluded.size
    ;`, f.ID.String(), f.Name, f.Path, f.Size)

	return f, err
}
