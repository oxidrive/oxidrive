package web

import (
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

func validator(_ zerolog.Logger) (api.MiddlewareFunc, error) {
	spec, err := api.GetSwagger()
	if err != nil {
		return nil, err
	}

	validator := middleware.OapiRequestValidatorWithOptions(spec, &middleware.Options{})
	return api.MiddlewareFunc(validator), nil
}
