package sqlite

import (
	"github.com/jmoiron/sqlx"
	_ "modernc.org/sqlite"
)

func Init(url string) (*sqlx.DB, error) {
	return sqlx.Connect("sqlite", url)
}
