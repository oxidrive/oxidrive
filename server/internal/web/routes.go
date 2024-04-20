package web

import (
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/routes/api"
)

func routes(cfg *Config) *http.ServeMux {
	router := http.NewServeMux()

	// Routes
	router.Handle("POST /api/setup", api.Setup(cfg.Logger.With().Str("handler", "api.setup").Logger(), cfg.Application))
	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router
}
