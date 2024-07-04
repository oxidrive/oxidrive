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

type SessionsTestsInit func(t *testing.T, ctx context.Context) auth.Sessions

func Sessions(t *testing.T, dep testutil.IntegrationDependency, init SessionsTestsInit) {
	exp := time.Now().Add(1 * time.Hour)

	t.Run("stores and returns a session by ID", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		sessions := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		session := testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))

		found, err := sessions.ByID(ctx, session.Value)

		assert.NoError(t, err)
		assert.Equal(t, session.Value, found.Value)
		assert.Equal(t, session.UserID, found.UserID)
		assert.Equal(t, session.ExpiresAt.UTC().Truncate(time.Second), found.ExpiresAt.UTC().Truncate(time.Second))
	})

	t.Run("returns the list of all expiring sessions", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		sessions := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(auth.NewSession(u, exp))
		t1.ExpiresAt = time.Now().Add(-1 * time.Hour)
		t1 = testutil.Must(sessions.Store(ctx, *t1))
		_ = testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))

		tt, err := sessions.ExpiringBefore(ctx, time.Now())

		assert.NoError(t, err)
		assert.Len(t, tt, 1)
		assert.Equal(t, t1.Value, tt[0].Value)
	})

	t.Run("deletes some sessions", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		sessions := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))
		t2 := testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))

		err := sessions.DeleteAll(ctx, []auth.Session{*t1, *t2})
		require.NoError(t, err)

		t1, err = sessions.ByID(ctx, t1.Value)
		assert.NoError(t, err)
		assert.Nil(t, t1)

		t2, err = sessions.ByID(ctx, t2.Value)
		assert.NoError(t, err)
		assert.Nil(t, t2)
	})

	t.Run("doesn't delete anything if no sessions are provided", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		sessions := init(t, ctx)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))
		t2 := testutil.Must(sessions.Store(ctx, *testutil.Must(auth.NewSession(u, exp))))

		err := sessions.DeleteAll(ctx, []auth.Session{})
		require.NoError(t, err)

		t1, err = sessions.ByID(ctx, t1.Value)
		assert.NoError(t, err)
		assert.NotNil(t, t1)

		t2, err = sessions.ByID(ctx, t2.Value)
		assert.NoError(t, err)
		assert.NotNil(t, t2)
	})
}
