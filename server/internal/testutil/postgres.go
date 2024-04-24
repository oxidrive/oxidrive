package testutil

import (
	"context"
	"errors"
	"log"
	"net/url"
	"testing"
	"time"

	"github.com/golang-migrate/migrate/v4"
	"github.com/jmoiron/sqlx"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/modules/postgres"
	"github.com/testcontainers/testcontainers-go/wait"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/migrations"

	_ "github.com/jackc/pgx/v5/stdlib"
)

const (
	pgUser     = "oxidrive"
	pgPassword = "oxidrive"
	pgName     = "oxidrive"
)

// / WithPgDB creates a temporary Postgres database
func WithPgDB() IntegrationDependency {
	return IntegrationDependency(func(ctx context.Context, t *testing.T) (context.Context, func()) {
		t.Helper()

		pg, err := postgres.RunContainer(ctx,
			testcontainers.WithImage("public.ecr.aws/docker/library/postgres:16-alpine"),
			postgres.WithDatabase(pgName),
			postgres.WithUsername(pgUser),
			postgres.WithPassword(pgPassword),
			testcontainers.WithLogger(testcontainers.TestLogger(t)),
			testcontainers.WithWaitStrategy(
				wait.ForLog(".*database system is ready to accept connections").
					AsRegexp().
					WithOccurrence(2).
					WithStartupTimeout(60*time.Second)),
		)
		if err != nil {
			log.Fatal(err)
		}

		url := pg.MustConnectionString(ctx, "sslmode=disable")
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
	url, ok := ctx.Value(pgKey{}).(string)
	if !ok {
		t.Fatal("failed to cast Postgres database URL from context to string")
	}

	if url == "" {
		t.Fatal("Postgres database URL not found in context")
	}

	return url
}

func PgDBFromContext(ctx context.Context, t *testing.T) *sqlx.DB {
	u := PgUrlFromContext(ctx, t)
	url, err := url.Parse(u)
	if err != nil {
		t.Fatal(err)
	}

	cfg := config.DatabaseConfig{Url: url}

	if err := migrations.Run(cfg); err != nil && !errors.Is(err, migrate.ErrNoChange) {
		t.Fatal(err)
	}

	db, err := sqlx.Connect(cfg.DatabaseDriver(), cfg.DatabaseSource())
	if err != nil {
		t.Fatal(err)
	}

	return db
}
