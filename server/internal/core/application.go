package core

import (
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

func NewApplication(deps ApplicationDependencies) *Application {
	return &Application{
		instance: instance.NewService(deps.Users),
		file:     file.NewService(deps.FilesContent, deps.FilesMetadata),
	}
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}

func (app *Application) File() *file.Service {
	return &app.file
}
