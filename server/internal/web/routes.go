package web

import (
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/middleware"
	"github.com/oxidrive/oxidrive/server/internal/web/route/api"
)

func routes(cfg *Config) *http.ServeMux {
	router := http.NewServeMux()

	// Routes
	router.Handle(
		"POST /api/setup",
		middleware.Apply(
			api.Setup(cfg.Logger.With().Str("handler", "api.setup").Logger(), cfg.Application),
			middleware.EnforceContentType("application/json"),
		),
	)
	router.Handle(
		"POST /api/files",
		middleware.Apply(
			api.Upload(cfg.Logger.With().Str("handler", "api.upload").Logger(), cfg.Application),
			middleware.EnforceContentType("multipart/form-data"),
			middleware.Authenticate(),
		),
	)
	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router
}
