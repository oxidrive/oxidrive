package web

import (
	"context"

	"github.com/getkin/kin-openapi/openapi3filter"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type MiddlewareFactory func(zerolog.Logger) (api.MiddlewareFunc, error)

func defaultMiddlewares(logger zerolog.Logger) ([]api.MiddlewareFunc, error) {
	middlewares := make([]api.MiddlewareFunc, 1)

	for i, mf := range []MiddlewareFactory{validator} {
		m, err := mf(logger)
		if err != nil {
			return nil, err
		}

		middlewares[i] = m
	}

	return middlewares, nil
}

func validator(logger zerolog.Logger) (api.MiddlewareFunc, error) {
	logger = logger.With().Str("middleware", "oapi_request_authentication").Logger()

	spec, err := api.GetSwagger()
	if err != nil {
		return nil, err
	}

	validator := middleware.OapiRequestValidatorWithOptions(spec, &middleware.Options{
		Options: openapi3filter.Options{
			AuthenticationFunc: func(ctx context.Context, auth *openapi3filter.AuthenticationInput) error {
				logger.Info().Interface("auth", auth)
				return nil
			},
		},
		SilenceServersWarning: false,
	})
	return api.MiddlewareFunc(validator), nil
}
