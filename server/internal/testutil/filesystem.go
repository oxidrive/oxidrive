package testutil

import (
	"context"
	"os"
	"testing"
)

// / WithTempDir creates a tempdir using the std testing lib
func WithTempDir() IntegrationDependency {
	return IntegrationDependency(func(ctx context.Context, t *testing.T) (context.Context, func()) {
		t.Helper()

		dir, err := os.MkdirTemp("", "")
		if err != nil {
			t.Fatal(err)
		}

		ctx = context.WithValue(ctx, dirKey{}, dir)
		return ctx, func() {
			if err := os.RemoveAll(dir); err != nil {
				t.Fatal(err)
			}
		}
	})
}

type dirKey struct{}

// / TempDirFromContext returns the path to the generated tempdir
func TempDirFromContext(ctx context.Context) string {
	dir, ok := ctx.Value(dirKey{}).(string)
	if !ok {
		panic("failed to cast tempdir from context to string")
	}

	if dir == "" {
		panic("tempdir not found in context")
	}

	return dir
}
