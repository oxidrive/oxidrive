package file

import (
	"context"

	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type SqliteFiles struct {
	db *sqlx.DB
}

func NewSqliteFiles(db *sqlx.DB) *SqliteFiles {
	return &SqliteFiles{db: db}
}

func (p *SqliteFiles) Save(ctx context.Context, u user.User, f file.File, logger zerolog.Logger) (*file.File, error) {
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
    ;`, f.ID.String(), f.Name, f.Path, f.Size, u.ID.String()); err != nil {
		return nil, err
	}

	return &f, nil
}
