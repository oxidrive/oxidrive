package user

import (
	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type SqliteUsers struct {
	db *sqlx.DB
}

func NewSqliteUsers(db *sqlx.DB) *SqliteUsers {
	return &SqliteUsers{db: db}
}

func (p *SqliteUsers) Count() (int, error) {
	var count int
	err := p.db.Get(&count, "select count(id) from users")
	return count, err
}

func (p *SqliteUsers) Save(u user.User) (user.User, error) {
	_, err := p.db.Exec(`insert into users (
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
    ;`, u.Id.String(), u.Username, u.PasswordHash.Expose())
	return u, err
}
