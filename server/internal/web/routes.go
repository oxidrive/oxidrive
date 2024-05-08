package web

import (
	"net/http"
)

func Router(cfg *Config) (*http.ServeMux, error) {
	router := http.NewServeMux()

	// Routes
	if err := mountApi(router, cfg); err != nil {
		return nil, err
	}

	router.Handle("/", serveFrontend(cfg.FrontendFolder))

	return router, nil
}
