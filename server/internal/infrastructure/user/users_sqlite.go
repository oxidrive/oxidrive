package user

import (
	"context"
	"database/sql"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/password"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ user.Users = (*SqliteUsers)(nil)

type SqliteUsers struct {
	db *sqlx.DB
}

func NewSqliteUsers(db *sqlx.DB) *SqliteUsers {
	return &SqliteUsers{db: db}
}

func (p *SqliteUsers) Count(ctx context.Context) (int, error) {
	var count int
	err := p.db.GetContext(ctx, &count, "select count(id) from users")
	return count, err
}

func (p *SqliteUsers) Save(ctx context.Context, u user.User) (*user.User, error) {
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

func (p *SqliteUsers) ByID(ctx context.Context, id user.ID) (*user.User, error) {
	u := new(sqliteUser)
	err := p.db.GetContext(ctx, u, "select id, username, password_hash from users where id = $1", id.String())
	if err == sql.ErrNoRows {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return u.into(), nil
}

func (p *SqliteUsers) ByUsername(ctx context.Context, username string) (*user.User, error) {
	u := new(sqliteUser)
	err := p.db.GetContext(ctx, u, "select id, username, password_hash from users where username = $1", username)
	if err == sql.ErrNoRows {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return u.into(), nil
}

type sqliteUser struct {
	ID           uuid.UUID `db:"id"`
	Username     string    `db:"username"`
	PasswordHash []byte    `db:"password_hash"`
}

func (pu *sqliteUser) into() *user.User {
	return &user.User{
		ID:           user.ID(pu.ID),
		Username:     pu.Username,
		PasswordHash: password.Must(pu.PasswordHash),
	}
}
