package web

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"strings"

	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/go-http-utils/headers"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

var (
	ErrTokenAuthenticationFailed = errors.New("token authentication failed")
)

func authenticateOpenAPI(logger zerolog.Logger, app *app.Application) openapi3filter.AuthenticationFunc {
	return func(ctx context.Context, input *openapi3filter.AuthenticationInput) error {
		var auth authenticator
		switch input.SecuritySchemeName {
		case "token":
			auth = tokenAuthenticator{
				logger: logger.With().Str("authenticator", "token").Logger(),
				app:    app,
			}
		default:
			return fmt.Errorf("unsupported authentication scheme: %s", input.SecuritySchemeName)
		}

		return auth.authenticate(ctx, input.RequestValidationInput.Request)
	}
}

func authenticateHttp(logger zerolog.Logger, app *app.Application) api.MiddlewareFunc {
	auth := tokenAuthenticator{
		logger: logger.With().Str("authenticator", "token").Logger(),
		app:    app,
	}

	inject := injectUserFromRequest(app)

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

type tokenAuthenticator struct {
	logger zerolog.Logger
	app    *app.Application
}

func (t tokenAuthenticator) authenticate(ctx context.Context, req *http.Request) error {
	token := extractTokenFromRequest(req)
	if token == "" {
		return ErrTokenAuthenticationFailed
	}

	if err := t.app.Tokens().Verify(ctx, auth.TokenID(token)); err != nil {
		t.logger.Debug().Err(err).Msg("token authentication failed")
		return ErrTokenAuthenticationFailed
	}

	t.logger.Trace().Msg("authentication successful")
	return nil
}

func extractTokenFromRequest(r *http.Request) string {
	authorization := r.Header.Get(headers.Authorization)
	return strings.TrimSpace(strings.Replace(authorization, "Bearer", "", 1))
}
