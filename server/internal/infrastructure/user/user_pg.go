package user

import (
	"context"

	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type PgUsers struct {
	db *sqlx.DB
}

func NewPgUsers(db *sqlx.DB) *PgUsers {
	return &PgUsers{db: db}
}

func (p *PgUsers) Count(ctx context.Context) (int, error) {
	var count int
	err := p.db.GetContext(ctx, &count, "select count(id) from users")
	return count, err
}

func (p *PgUsers) Save(ctx context.Context, u user.User) (*user.User, error) {
	_, err := p.db.ExecContext(ctx, `insert into users (
        id,
        username,
        password_hash
    ) values (
        $1,
        $2,
        $3
    )
    on conflict (id)
    do update set
      username = excluded.username,
      password_hash = excluded.password_hash
    ;`, u.ID.String(), u.Username, u.PasswordHash.Expose())
	if err != nil {
		return nil, err
	}

	return &u, nil
}
