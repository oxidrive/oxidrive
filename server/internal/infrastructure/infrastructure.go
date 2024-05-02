package infrastructure

import (
	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	fileinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/file"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
)

func Setup(cfg config.Config, db *sqlx.DB, logger zerolog.Logger) core.ApplicationDependencies {
	var users user.Users
	var contents file.Contents
	var files file.Files

	switch db.DriverName() {
	case config.DriverPG:
		users = userinfra.NewPgUsers(db)
		files = fileinfra.NewPgFiles(db)
	case config.DriverSqlite:
		users = userinfra.NewSqliteUsers(db)
		files = fileinfra.NewSqliteFiles(db)
	}

	contents = fileinfra.NewContentFS(cfg.StorageConfig, logger)

	return core.ApplicationDependencies{
		Users:    users,
		Files:    files,
		Contents: contents,
	}

}

func initSqliteDB(db *sqlx.DB) (*sqlx.DB, error) {
	_, err := db.Exec("PRAGMA foreign_keys = ON;")

	return db, err
}

func InitDB(cfg config.DatabaseConfig) (*sqlx.DB, error) {
	driver := cfg.DatabaseDriver()
	db, err := sqlx.Connect(driver, cfg.DatabaseSource())
	if err != nil {
		return nil, err
	}

	switch driver {
	case config.DriverSqlite:
		db, err = initSqliteDB(db)
		if err != nil {
			return nil, err
		}
	}

	return db, nil
}
