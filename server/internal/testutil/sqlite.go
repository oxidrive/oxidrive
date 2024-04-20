package testutil

import (
	"context"
	"fmt"
	"os"
	"path"
	"testing"
)

type SqliteDBConfig struct {
	DbName   string
	DbParams string
}

// / WithSqliteDB creates a temporary SQLite database
func WithSqliteDB(cfg SqliteDBConfig) IntegrationDependency {
	return IntegrationDependency(func(ctx context.Context, t *testing.T) (context.Context, func()) {
		t.Helper()

		dir, err := os.MkdirTemp("", "")
		if err != nil {
			t.Fatal(err)
		}

		if cfg.DbName == "" {
			cfg.DbName = "oxidrive.db"
		}

		url := fmt.Sprintf("sqlite://%s?%s", path.Join(dir, cfg.DbName), cfg.DbParams)

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
func SqliteUrlFromContext(ctx context.Context) string {
	dir, ok := ctx.Value(sqliteKey{}).(string)
	if !ok {
		panic("failed to cast SQLite database URL from context to string")
	}

	if dir == "" {
		panic("SQLite database URL not found in context")
	}

	return dir
}
