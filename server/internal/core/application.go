package core

import (
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Application struct {
	instance instance.Service
	files    file.Service
	users    user.Users
}

type ApplicationDependencies struct {
	Users    user.Users
	Files    file.Files
	Contents file.Contents
}

func NewApplication(cfg config.Config, deps ApplicationDependencies) *Application {
	return &Application{
		instance: instance.InitService(instance.Info{
			PublicURL:   cfg.PublicURL,
			Database:    instance.StatusDB(cfg.DatabaseName()),
			FileStorage: instance.StatusFileStorageFS, // TODO: add real file store
		}, deps.Users),
		files: file.InitService(deps.Contents, deps.Files),
		users: deps.Users,
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}

func (app *Application) Files() *file.Service {
	return &app.files
}

func (app *Application) Users() user.Users {
	return app.users
}
