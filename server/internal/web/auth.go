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

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core"
)

var (
	ErrTokenAuthenticationFailed = errors.New("token authentication failed")
)

func authenticate(logger zerolog.Logger, app *core.Application) openapi3filter.AuthenticationFunc {
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

		return auth.authenticate(ctx, input)
	}
}

type authenticator interface {
	authenticate(ctx context.Context, input *openapi3filter.AuthenticationInput) error
}

type tokenAuthenticator struct {
	logger zerolog.Logger
	app    *core.Application
}

func (t tokenAuthenticator) authenticate(ctx context.Context, input *openapi3filter.AuthenticationInput) error {
	token := extractTokenFromRequest(input.RequestValidationInput.Request)
	if token == "" {
		return ErrTokenAuthenticationFailed
	}

	if err := t.app.TokenVerifier().VerifyToken(ctx, auth.TokenID(token)); err != nil {
		t.logger.Warn().Err(err).Msg("token authentication failed")
		return ErrTokenAuthenticationFailed
	}

	t.logger.Debug().Msg("authentication successful")
	return nil
}

func extractTokenFromRequest(r *http.Request) string {
	authorization := r.Header.Get(headers.Authorization)
	return strings.TrimSpace(strings.Replace(authorization, "Bearer", "", 1))
}
