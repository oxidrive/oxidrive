package web

import (
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/route/api"
)

func routes(cfg *Config) *http.ServeMux {
	router := http.NewServeMux()

	// Routes
	router.Handle("POST /api/setup", api.Setup(cfg.Logger.With().Str("handler", "api.setup").Logger(), cfg.Application))
	router.Handle("POST /api/upload", api.Setup(cfg.Logger.With().Str("handler", "api.upload").Logger(), cfg.Application))
	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router
}
