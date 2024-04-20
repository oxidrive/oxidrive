package core

import (
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Application struct {
	instance instance.Service
}

type ApplicationDependencies struct {
	Users user.Users
}

func NewApplication(deps ApplicationDependencies) *Application {
	return &Application{
		instance: instance.NewService(deps.Users),
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}
