package core

import (
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Application struct {
	instance instance.Service
}

type ApplicationDependencies struct {
	Users user.Users
}

func NewApplication(cfg config.Config, deps ApplicationDependencies) *Application {
	return &Application{
		instance: instance.NewService(instance.Info{
			PublicURL:   cfg.PublicURL,
			Database:    instance.StatusDB(cfg.DatabaseName()),
			FileStorage: instance.StatusFileStorageFS, // TODO: add real file store
		}, deps.Users),
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}
