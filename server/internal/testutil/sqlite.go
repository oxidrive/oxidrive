package testutil

import (
	"context"
	"fmt"
	"net/url"
	"os"
	"path"
	"testing"

	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/sqlite"
	"github.com/oxidrive/oxidrive/server/migrations"
)

const dbName = "oxidrive.db"

// / WithSqliteDB creates a temporary SQLite database
func WithSqliteDB() IntegrationDependency {
	return IntegrationDependency(func(ctx context.Context, t *testing.T) (context.Context, func()) {
		t.Helper()

		dir, err := os.MkdirTemp("", "")
		if err != nil {
			t.Fatal(err)
		}

		url := fmt.Sprintf("sqlite://%s", path.Join(dir, dbName))

		ctx = context.WithValue(ctx, sqliteKey{}, url)
		return ctx, func() {
			if err := os.RemoveAll(dir); err != nil {
				t.Fatal(err)
			}
		}
	})
}

type sqliteKey struct{}

// / SqliteUrlFromContext returns the database URL for the teemporary SQLite database
func SqliteUrlFromContext(ctx context.Context, t *testing.T) string {
	dir, ok := ctx.Value(sqliteKey{}).(string)
	if !ok {
		t.Fatal("failed to cast SQLite database URL from context to string")
	}

	if dir == "" {
		t.Fatal("SQLite database URL not found in context")
	}

	return dir
}

func SqliteDBFromContext(ctx context.Context, t *testing.T) *sqlx.DB {
	sqlite.Init()

	u := SqliteUrlFromContext(ctx, t)
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

	_, err = db.Exec("PRAGMA foreign_keys = ON;")
	if err != nil {
		t.Fatal(err)
	}

	return db
}
