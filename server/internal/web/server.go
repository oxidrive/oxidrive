package web

import (
	"net/http"

	"github.com/oxidrive/oxidrive/internal/application"
	"github.com/rs/zerolog"
)

type Config struct {
	Address     string
	Application *application.Application
	Logger      zerolog.Logger
}

func Run(cfg Config) error {
	router := routes()

	cfg.Logger.Info().Str("listen", cfg.Address).Msg("starting oxidrive server")
	return http.ListenAndServe(cfg.Address, router)
}
