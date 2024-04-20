package user

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestPgUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB(testutil.PgDBConfig{}))
		defer done()

		_ = testutil.PgDBFromContext(ctx, t)

	})
}

func TestPgUsers_Save(t *testing.T) {
	t.Run("creates a new user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB(testutil.PgDBConfig{}))
		defer done()

		_ = testutil.PgDBFromContext(ctx, t)
	})

	t.Run("updates an existing user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB(testutil.PgDBConfig{}))
		defer done()

		_ = testutil.PgDBFromContext(ctx, t)
	})
}
