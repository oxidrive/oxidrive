package web

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
)

type Config struct {
	Address        string
	Application    *core.Application
	Logger         zerolog.Logger
	FrontendFolder string
}

func Run(cfg Config) error {
	router := routes(&cfg)

	cfg.Logger.Info().Str("listen", cfg.Address).Msg("starting oxidrive server")
	return http.ListenAndServe(cfg.Address, router)
}
