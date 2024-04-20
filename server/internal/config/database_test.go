package config

import (
	"net/url"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPostgresConfig(t *testing.T) {
	url, err := url.Parse("postgres://test:test@test:5432/test?sslmode=disable")
	assert.NoError(t, err)

	t.Run("forwards the given URL", func(t *testing.T) {
		cfg := DatabaseConfig{Url: url}
		assert.Equal(t, url, cfg.DatabaseUrl())
	})

	t.Run("assembles a URL from single params", func(t *testing.T) {
		cfg := DatabaseConfig{
			PostgresHost:     "test",
			PostgresPort:     "5432",
			PostgresUser:     "test",
			PostgresPassword: "test",
			PostgresDB:       "test",
			PostgresArgs:     "sslmode=disable",
		}

		assert.Equal(t, url, cfg.DatabaseUrl())
	})

	t.Run("returns the correct datasource", func(t *testing.T) {
		cfg := DatabaseConfig{Url: url}
		assert.Equal(t, url.String(), cfg.DatabaseSource())
	})
}

func TestSqliteConfig(t *testing.T) {
	url, err := url.Parse("sqlite://:memory:")
	assert.NoError(t, err)

	t.Run("forwards the given URL", func(t *testing.T) {
		cfg := DatabaseConfig{Url: url}
		assert.Equal(t, url, cfg.DatabaseUrl())
	})

	t.Run("returns the correct datasource", func(t *testing.T) {
		cfg := DatabaseConfig{Url: url}
		assert.Equal(t, ":memory:", cfg.DatabaseSource())
	})
}
