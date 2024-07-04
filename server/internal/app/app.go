package app

import (
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Application struct {
	auth     auth.Authenticator
	files    file.Service
	instance instance.Service
	users    user.Users
	sessions auth.SessionService
}

type ApplicationDependencies struct {
	Contents file.Contents
	Files    file.Files
	Users    user.Users
	Sessions auth.Sessions
}

func NewApplication(cfg config.Config, deps ApplicationDependencies) *Application {
	sessions := auth.NewSessionService(deps.Sessions, cfg.SessionDuration)
	return &Application{
		auth:  auth.NewAuthenticator(deps.Users, sessions),
		files: file.NewService(deps.Contents, deps.Files),
		instance: instance.NewService(instance.Info{
			PublicURL:   cfg.PublicURL,
			Database:    instance.StatusDB(cfg.DatabaseName()),
			FileStorage: instance.StatusFileStorageFS, // TODO: add real file store
		}, deps.Users),
		users:    deps.Users,
		sessions: sessions,
	}
}

func (app *Application) Auth() *auth.Authenticator {
	return &app.auth
}

func (app *Application) Files() *file.Service {
	return &app.files
}

func (app *Application) Instance() *instance.Service {
	return &app.instance
}

func (app *Application) Users() user.Users {
	return app.users
}

func (app *Application) Sessions() *auth.SessionService {
	return &app.sessions
}
