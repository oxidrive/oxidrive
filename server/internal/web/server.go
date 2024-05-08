package web

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
)

type Config struct {
	Address            string
	Application        *app.Application
	Logger             zerolog.Logger
	FrontendFolder     string
	MultipartMaxMemory int64
}

func Run(cfg Config) error {
	router, err := Router(&cfg)
	if err != nil {
		return err
	}

	cfg.Logger.Info().Str("listen", cfg.Address).Msg("starting oxidrive server")
	return http.ListenAndServe(cfg.Address, router)
}
