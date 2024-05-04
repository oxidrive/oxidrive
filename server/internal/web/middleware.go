package web

import (
	"context"
	"net/http"

	"github.com/getkin/kin-openapi/openapi3filter"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/oapi-codegen/runtime/strictmiddleware/nethttp"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type MiddlewareFactory func(zerolog.Logger) (api.MiddlewareFunc, error)

func defaultMiddlewares(logger zerolog.Logger, app *core.Application) ([]api.MiddlewareFunc, error) {
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

func validator(app *core.Application) MiddlewareFactory {
	return func(logger zerolog.Logger) (api.MiddlewareFunc, error) {
		spec, err := api.GetSwagger()
		if err != nil {
			return nil, err
		}

		validator := middleware.OapiRequestValidatorWithOptions(spec, &middleware.Options{
			Options: openapi3filter.Options{
				AuthenticationFunc: authenticate(logger, app),
			},
		})
		return api.MiddlewareFunc(validator), nil
	}
}

func userFromToken(app *core.Application) api.StrictMiddlewareFunc {
	return api.StrictMiddlewareFunc(func(f nethttp.StrictHTTPHandlerFunc, operationID string) nethttp.StrictHTTPHandlerFunc {
		return func(ctx context.Context, w http.ResponseWriter, r *http.Request, request interface{}) (response interface{}, err error) {
			token := extractTokenFromRequest(r)
			if token == "" {
				return f(ctx, w, r, request)
			}

			u, err := app.Auth().UserForToken(ctx, auth.TokenID(token))
			if err != nil {
				return nil, err
			}

			if u == nil {
				panic("current user from token was nil but this is impossible, as it should have been validated by the authentication middleware!")
			}

			ctx = auth.WithCurrentUser(ctx, u)
			return f(ctx, w, r, request)
		}
	})
}
