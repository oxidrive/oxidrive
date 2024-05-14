package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestSqliteTokens(t *testing.T) {
	exp := time.Now().Add(1 * time.Hour)

	t.Run("stores and returns a token by ID", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		tokens := NewSqliteTokens(db)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))
		testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		found, err := tokens.ByID(ctx, t1.Value)

		assert.NoError(t, err)
		assert.Equal(t, t1.Value, found.Value)
		assert.Equal(t, t1.UserID, found.UserID)
		assert.Equal(t, t1.ExpiresAt.UTC().Truncate(time.Second), found.ExpiresAt.UTC().Truncate(time.Second))
	})

	t.Run("returns the list of all expiring tokens", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		tokens := NewSqliteTokens(db)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(auth.NewToken(u, exp))
		t1.ExpiresAt = time.Now().Add(-1 * time.Hour)
		t1 = testutil.Must(tokens.Store(ctx, *t1))
		_ = testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		tt, err := tokens.ExpiringBefore(ctx, time.Now())

		assert.NoError(t, err)
		assert.Len(t, tt, 1)
		assert.Equal(t, t1.Value, tt[0].Value)
	})

	t.Run("deletes some tokens", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		tokens := NewSqliteTokens(db)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))
		_ = testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		err := tokens.DeleteAll(ctx, []auth.Token{*t1})
		require.NoError(t, err)

		t1, err = tokens.ByID(ctx, t1.Value)
		assert.NoError(t, err)
		assert.Nil(t, t1)
	})

	t.Run("doesn't delete anything if no tokens are provided", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		tokens := NewSqliteTokens(db)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))
		t2 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		err := tokens.DeleteAll(ctx, []auth.Token{})
		require.NoError(t, err)

		t1, err = tokens.ByID(ctx, t1.Value)
		assert.NoError(t, err)
		assert.NotNil(t, t1)

		t2, err = tokens.ByID(ctx, t2.Value)
		assert.NoError(t, err)
		assert.NotNil(t, t2)
	})
}
