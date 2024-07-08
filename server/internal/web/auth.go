package web

import (
	"context"
	"errors"
	"fmt"
	"net/http"

	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

var (
	ErrSessionAuthenticationFailed = errors.New("session authentication failed")
)

func authenticateOpenAPI(logger zerolog.Logger, app *app.Application) openapi3filter.AuthenticationFunc {
	return func(ctx context.Context, input *openapi3filter.AuthenticationInput) error {
		logger.Debug().
			Str("securitySchemeName", input.SecuritySchemeName).
			Interface("securityScheme", input.SecurityScheme).
			Msg("verifying authentication")

		var auth authenticator
		switch input.SecuritySchemeName {
		case "session":
			auth = sessionAuthenticator{
				logger: logger.With().Str("authenticator", "session").Logger(),
				app:    app,
			}
		case "":
			return errors.New("security scheme name is empty")
		default:
			return fmt.Errorf("unsupported authentication scheme: %s", input.SecuritySchemeName)
		}

		return auth.authenticate(ctx, input.RequestValidationInput.Request)
	}
}

func authenticateHttp(logger zerolog.Logger, app *app.Application) api.MiddlewareFunc {
	auth := sessionAuthenticator{
		logger: logger.With().Str("authenticator", "session").Logger(),
		app:    app,
	}

	inject := injectSessionFromRequest(app)

	return func(h http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			ctx := r.Context()

			if err := auth.authenticate(ctx, r); err != nil {
				http.Error(w, err.Error(), http.StatusUnauthorized)
				return
			}

			ctx, err := inject(ctx, r)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}

			r = r.WithContext(ctx)

			h.ServeHTTP(w, r)
		})
	}
}

type authenticator interface {
	authenticate(ctx context.Context, req *http.Request) error
}

type sessionAuthenticator struct {
	logger zerolog.Logger
	app    *app.Application
}

func (t sessionAuthenticator) authenticate(ctx context.Context, req *http.Request) error {
	session := extractSessionID(req)
	if session == "" {
		return ErrSessionAuthenticationFailed
	}

	if err := t.app.Sessions().Verify(ctx, session); err != nil {
		t.logger.Debug().Err(err).Msg("session authentication failed")
		return ErrSessionAuthenticationFailed
	}

	t.logger.Trace().Msg("authentication successful")
	return nil
}

func extractSessionID(r *http.Request) auth.SessionID {
	c, err := r.Cookie(handler.SessionCookieName)
	if err != nil {
		return ""
	}
	return auth.SessionID(c.Value)
}
