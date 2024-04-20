package testutil

import (
	"context"
	"fmt"
	"net/url"
	"testing"

	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/migrations"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
)

type PgDBConfig struct {
	DbParams string
}

const (
	pgUser     = "oxidrive"
	pgPassword = "oxidrive"
	pgName     = "oxidrive"
)

// / WithPgDB creates a temporary Postgres database
func WithPgDB(cfg PgDBConfig) IntegrationDependency {
	return IntegrationDependency(func(ctx context.Context, t *testing.T) (context.Context, func()) {
		t.Helper()

		req := testcontainers.ContainerRequest{
			Image: "public.ecr.aws/docker/library/postgres:15-alpine",
			Env: map[string]string{
				"POSTGRES_USER":     pgUser,
				"POSTGRES_PASSWORD": pgPassword,
				"POSTGRES_DB":       pgName,
			},
			ExposedPorts: []string{"5432/tcp"},
			WaitingFor:   wait.ForLog("database system is ready to accept connections"),
		}
		pg, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{ContainerRequest: req, Started: true})
		if err != nil {
			t.Fatal(err)
		}

		host, err := pg.Host(ctx)
		if err != nil {
			t.Fatal(err)
		}

		port, err := pg.MappedPort(ctx, "5432")
		if err != nil {
			t.Fatal(err)
		}

		url := fmt.Sprintf("postgres://%s:%s@%s:%s/%s?sslmode=disable&%s", pgUser, pgPassword, host, port, pgName, cfg.DbParams)
		ctx = context.WithValue(ctx, pgKey{}, url)
		return ctx, func() {
			if err := pg.Terminate(ctx); err != nil {
				t.Fatal(err)
			}
		}
	})
}

type pgKey struct{}

// / PgUrlFromContext returns the database URL for the teemporary SQLite database
func PgUrlFromContext(ctx context.Context, t *testing.T) string {
	dir, ok := ctx.Value(pgKey{}).(string)
	if !ok {
		t.Fatal("failed to cast Postgres database URL from context to string")
	}

	if dir == "" {
		t.Fatal("Postgres database URL not found in context")
	}

	return dir
}

func PgDBFromContext(ctx context.Context, t *testing.T) *sqlx.DB {
	u := PgUrlFromContext(ctx, t)
	url, err := url.Parse(u)
	if err != nil {
		t.Fatal(err)
	}

	cfg := config.DatabaseConfig{Url: url}

	if err := migrations.Run(cfg); err != nil {
		t.Fatal(err)
	}

	db, err := sqlx.Connect(cfg.DatabaseDriver(), cfg.DatabaseSource())
	if err != nil {
		t.Fatal(err)
	}

	return db
}
