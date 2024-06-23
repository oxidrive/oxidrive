package web

import (
	"context"
	"net/http"

	"github.com/getkin/kin-openapi/openapi3filter"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/oapi-codegen/runtime/strictmiddleware/nethttp"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type MiddlewareFactory func(zerolog.Logger) (api.MiddlewareFunc, error)

func defaultMiddlewares(logger zerolog.Logger, app *app.Application) ([]api.MiddlewareFunc, error) {
	middlewares := make([]api.MiddlewareFunc, 1)

	for i, mf := range []MiddlewareFactory{validator(app)} {
		m, err := mf(logger)
		if err != nil {
			return nil, err
		}

		middlewares[i] = m
	}

	return middlewares, nil
}

func validator(app *app.Application) MiddlewareFactory {
	return func(l zerolog.Logger) (api.MiddlewareFunc, error) {
		spec, err := api.GetSwagger()
		if err != nil {
			return nil, err
		}

		validator := middleware.OapiRequestValidatorWithOptions(spec, &middleware.Options{
			ErrorHandler: handleApiError(l.
				With().
				Str("lifecycle", "request").
				Str("middleware", "openapi-validator").
				Logger()),
			Options: openapi3filter.Options{
				AuthenticationFunc: authenticateOpenAPI(l.
					With().
					Str("lifecycle", "request").
					Str("middleware", "openapi-authenticator").
					Logger(),
					app),
			},
		})
		return api.MiddlewareFunc(validator), nil
	}
}

func userFromToken(app *app.Application) api.StrictMiddlewareFunc {
	inject := injectUserFromRequest(app)

	return api.StrictMiddlewareFunc(func(f nethttp.StrictHTTPHandlerFunc, operationID string) nethttp.StrictHTTPHandlerFunc {
		return func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (response interface{}, err error) {
			ctx, err = inject(ctx, r)
			if err != nil {
				return nil, err
			}

			return f(ctx, w, r, request)
		}
	})
}

func injectUserFromRequest(app *app.Application) func(ctx context.Context, r *http.Request) (context.Context, error) {
	return func(ctx context.Context, r *http.Request) (context.Context, error) {
		token := extractTokenFromCookie(r)
		if token == "" {
			return ctx, nil
		}

		u, err := app.Auth().UserForToken(ctx, auth.TokenID(token))
		if err != nil {
			return nil, err
		}

		if u == nil {
			return ctx, nil
		}

		return auth.WithCurrentUser(ctx, u), nil
	}
}
