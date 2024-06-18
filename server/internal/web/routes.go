package web

import (
	"net/http"
)

func Router(cfg *Config) (*http.ServeMux, error) {
	auth := authenticateHttp(cfg.Logger, cfg.Application)
	router := http.NewServeMux()

	// Routes
	if err := mountApi(router, cfg); err != nil {
		return nil, err
	}

	router.Handle("GET /blob/{path...}", auth(serveBlob(cfg.Application)))
	router.Handle("GET /", serveFrontend(cfg.FrontendFolder))

	return router, nil
}
