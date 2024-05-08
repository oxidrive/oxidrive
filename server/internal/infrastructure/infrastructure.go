package infrastructure

import (
	"github.com/jmoiron/sqlx"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	authinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/auth"
	fileinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/file"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
)

func Setup(cfg config.Config, db *sqlx.DB, logger zerolog.Logger) app.ApplicationDependencies {
	var contents file.Contents
	var files file.Files
	var tokens auth.Tokens
	var users user.Users

	switch db.DriverName() {
	case config.DriverPG:
		users = userinfra.NewPgUsers(db)
		files = fileinfra.NewPgFiles(db)
		tokens = authinfra.NewPgTokens(db)
	case config.DriverSqlite:
		users = userinfra.NewSqliteUsers(db)
		files = fileinfra.NewSqliteFiles(db)
		tokens = authinfra.NewSqliteTokens(db)
	}

	contents = fileinfra.NewContentFS(cfg.StorageConfig, logger)

	return app.ApplicationDependencies{
		Contents: contents,
		Files:    files,
		Tokens:   tokens,
		Users:    users,
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
