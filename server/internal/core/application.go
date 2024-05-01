package core

import (
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
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
		instance: instance.InitService(instance.Info{
			PublicURL:   cfg.PublicURL,
			Database:    instance.StatusDB(cfg.DatabaseName()),
			FileStorage: instance.StatusFileStorageFS, // TODO: add real file store
		}, deps.Users),
		file: file.InitService(deps.FilesContent, deps.FilesMetadata),
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}

func (app *Application) File() *file.Service {
	return &app.file
}
