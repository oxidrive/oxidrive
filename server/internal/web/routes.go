package web

import (
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/routes/api/instance"
)

func routes(cfg *Config) *http.ServeMux {
	router := http.NewServeMux()

	// Routes
	router.Handle("GET /api/instance", instance.Status(cfg.Logger.With().Str("handler", "api.status").Logger(), cfg.Application))
	router.Handle("POST /api/instance/setup", instance.Setup(cfg.Logger.With().Str("handler", "api.setup").Logger(), cfg.Application))
	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router
}
