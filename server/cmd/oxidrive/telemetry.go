package main

import (
	"io"
	"os"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/rs/zerolog"
)

func InitLogger(cfg *config.Config) zerolog.Logger {
	var out io.Writer = os.Stdout
	if cfg.LogFormat == config.FormatText {
		out = zerolog.ConsoleWriter{
			Out:        os.Stdout,
			TimeFormat: time.RFC3339,
		}
	}

	return zerolog.New(out).Level(cfg.LogLevel).With().Timestamp().Caller().Logger()
}
