package core

import (
	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/config"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	fileinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/file"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
)

type Application struct {
	instance instance.Service
	file     file.Service
}

type ApplicationDependencies struct {
	Users         user.Users
	FilesContent  file.FilesContent
	FilesMetadata file.FilesMetadata
}

func NewApplication(cfg config.Config, deps ApplicationDependencies) *Application {
	return &Application{
		instance: instance.NewService(instance.Info{
			PublicURL:   cfg.PublicURL,
			Database:    instance.StatusDB(cfg.DatabaseName()),
			FileStorage: instance.StatusFileStorageFS, // TODO: add real file store
		}, deps.Users),
		file: file.NewService(deps.FilesContent, deps.FilesMetadata),
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}

func (app *Application) File() *file.Service {
	return &app.file
}

func SetupDependencies(cfg config.Config, db *sqlx.DB) ApplicationDependencies {
	var users user.Users
	var filesContent file.FilesContent
	var filesMetadata file.FilesMetadata

	switch db.DriverName() {
	case config.DriverPG:
		users = userinfra.NewPgUsers(db)
		filesMetadata = fileinfra.NewPgFiles(db)
	case config.DriverSqlite:
		users = userinfra.NewSqliteUsers(db)
		filesMetadata = fileinfra.NewSqliteFiles(db)
	}

	filesContent = fileinfra.NewBlobFS(cfg.StorageConfig)

	return ApplicationDependencies{
		Users:         users,
		FilesMetadata: filesMetadata,
		FilesContent:  filesContent,
	}

}
