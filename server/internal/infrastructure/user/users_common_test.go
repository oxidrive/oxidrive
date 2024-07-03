package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

type UsersTestsInit func(t *testing.T, ctx context.Context) user.Users

func Count(t *testing.T, dep testutil.IntegrationDependency, init UsersTestsInit) {
	t.Run("returns the number of users", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("b", "b"))))

		count, err := users.Count(ctx)

		require.NoError(t, err)
		assert.Equal(t, 2, count)

	})
}

func Save(t *testing.T, dep testutil.IntegrationDependency, init UsersTestsInit) {
	t.Run("creates a new user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		username := "testusername"

		created, err := users.Save(ctx, *testutil.Must(user.Create(username, "a")))
		require.NoError(t, err)
		assert.Equal(t, username, created.Username)

	})

	t.Run("updates an existing user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		username := "testuser"

		created, err := users.Save(ctx, *testutil.Must(user.Create(username, "a")))
		require.NoError(t, err)
		assert.Equal(t, username, created.Username)

		changedUsername := "changed"
		created.Username = changedUsername

		updated, err := users.Save(ctx, *created)
		require.NoError(t, err)
		assert.Equal(t, created.ID, updated.ID)
		assert.Equal(t, changedUsername, updated.Username)

	})
}

func ByID(t *testing.T, dep testutil.IntegrationDependency, init UsersTestsInit) {
	t.Run("returns the correct user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		u := testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByID(ctx, u.ID)

		require.NoError(t, err)
		assert.Equal(t, u.ID, found.ID)
		assert.Equal(t, u.Username, found.Username)
	})

	t.Run("does not return a different user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)
		id := user.NewID()

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByID(ctx, id)

		require.NoError(t, err)
		assert.Nil(t, found)
	})
}

func ByUsername(t *testing.T, dep testutil.IntegrationDependency, init UsersTestsInit) {
	t.Run("returns the correct user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create(username, "a"))))

		found, err := users.ByUsername(ctx, username)

		require.NoError(t, err)
		assert.Equal(t, username, found.Username)
	})

	t.Run("does not return a different user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		users := init(t, ctx)

		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByUsername(ctx, username)

		require.NoError(t, err)
		assert.Nil(t, found)
	})
}
