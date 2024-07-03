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

type TokensTestsInit func(t *testing.T, ctx context.Context) auth.Tokens

func Tokens(t *testing.T, dep testutil.IntegrationDependency, init TokensTestsInit) {
	exp := time.Now().Add(1 * time.Hour)

	t.Run("stores and returns a token by ID", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		tokens := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		token := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		found, err := tokens.ByID(ctx, token.Value)

		assert.NoError(t, err)
		assert.Equal(t, token.Value, found.Value)
		assert.Equal(t, token.UserID, found.UserID)
		assert.Equal(t, token.ExpiresAt.UTC().Truncate(time.Second), found.ExpiresAt.UTC().Truncate(time.Second))
	})

	t.Run("returns the list of all expiring tokens", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		tokens := init(t, ctx)

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

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		tokens := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))
		t2 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.NewToken(u, exp))))

		err := tokens.DeleteAll(ctx, []auth.Token{*t1, *t2})
		require.NoError(t, err)

		t1, err = tokens.ByID(ctx, t1.Value)
		assert.NoError(t, err)
		assert.Nil(t, t1)

		t2, err = tokens.ByID(ctx, t2.Value)
		assert.NoError(t, err)
		assert.Nil(t, t2)
	})

	t.Run("doesn't delete anything if no tokens are provided", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		tokens := init(t, ctx)

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
