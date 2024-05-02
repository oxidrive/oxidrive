package user

import (
	"context"
	"database/sql"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/password"
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

func (p *PgUsers) ByUsername(ctx context.Context, username string) (*user.User, error) {
	u := new(pgUser)
	err := p.db.GetContext(ctx, u, "select id, username, password_hash from users where username = $1", username)
	if err == sql.ErrNoRows {
		return nil, user.ErrUserNotFound
	}

	if err != nil {
		return nil, err
	}

	return u.into(), nil
}

type pgUser struct {
	ID           uuid.UUID `db:"id"`
	Username     string    `db:"username"`
	PasswordHash []byte    `db:"password_hash"`
}

func (pu *pgUser) into() *user.User {
	return &user.User{
		ID:           user.ID(pu.ID),
		Username:     pu.Username,
		PasswordHash: password.Must(pu.PasswordHash),
	}
}
