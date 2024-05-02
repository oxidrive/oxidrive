package web

import (
	"net/http"
)

func routes(cfg *Config) *http.ServeMux {
	router := http.NewServeMux()

	// Routes
	mountApi(router, cfg)

	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router
}
