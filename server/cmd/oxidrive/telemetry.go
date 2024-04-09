package main

import (
	"io"
	"os"
	"time"

	"github.com/rs/zerolog"
)

func InitLogger(cfg *Config) zerolog.Logger {
	var out io.Writer = os.Stdout
	if cfg.LogFormat == FormatText {
		out = zerolog.ConsoleWriter{
			Out:        os.Stdout,
			TimeFormat: time.RFC3339,
		}
	}

	return zerolog.New(out).Level(cfg.LogLevel).With().Timestamp().Caller().Logger()
}
