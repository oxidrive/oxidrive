package web

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
)

type Config struct {
	Address            string
	Application        *core.Application
	Logger             zerolog.Logger
	FrontendFolder     string
	MultipartMaxMemory int64
}

func Run(cfg Config) error {
	router, err := routes(&cfg)
	if err != nil {
		return err
	}

	cfg.Logger.Info().Str("listen", cfg.Address).Msg("starting oxidrive server")
	return http.ListenAndServe(cfg.Address, router)
}
