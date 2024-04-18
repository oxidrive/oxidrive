package config

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPostgresConfig(t *testing.T) {
	url := "postgres://test:test@test:5432/test?sslmode=disable"

	t.Run("forwards the given URL", func(t *testing.T) {
		cfg := PostgresConfig{DatabaseUrl: url}
		assert.Equal(t, url, cfg.Url())
	})

	t.Run("assembles a URL from single params", func(t *testing.T) {
		cfg := PostgresConfig{
			PostgresHost:     "test",
			PostgresPort:     "5432",
			PostgresUser:     "test",
			PostgresPassword: "test",
			PostgresDB:       "test",
			PostgresArgs:     "sslmode=disable",
		}

		assert.Equal(t, url, cfg.Url())

	})
}
