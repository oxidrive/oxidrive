package pg

import "github.com/jmoiron/sqlx"

func Init(url string) (*sqlx.DB, error) {
	return sqlx.Connect("pgx", url)
}
