package main

import (
	"fmt"
	"path/filepath"

	"github.com/alecthomas/kong"
	"github.com/rs/zerolog"
)

type LogFormat string

const (
	FormatJson LogFormat = "json"
	FormatText LogFormat = "text"
)

type Config struct {
	LogLevel  zerolog.Level `env:"LOG_LEVEL" default:"info"`
	LogFormat LogFormat     `env:"LOG_FORMAT" default:"text"`

	Host string `env:"HOST" default:"127.0.0.1"`
	Port int16  `env:"PORT" default:"4000"`

	AssetsFolder string `env:"OXIDRIVE_ASSETS_FOLDER" default:"./assets"`
}

func ParseConfig() Config {
	cfg := Config{}
	_ = kong.Parse(&cfg)
	return cfg
}

func (c *Config) ListenAddress() string {
	return fmt.Sprintf("%s:%d", c.Host, c.Port)
}

func (c *Config) AssetsFolderPath() string {
	path, err := filepath.Abs(c.AssetsFolder)
	if err != nil {
		panic(err)
	}

	return path
}
