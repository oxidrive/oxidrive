package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestPgUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("b", "b"))))

		count, err := users.Count(ctx)

		require.NoError(t, err)
		assert.Equal(t, 2, count)

	})
}

func TestPgUsers_Save(t *testing.T) {
	t.Run("creates a new user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		username := "testusername"

		users := NewPgUsers(db)

		created, err := users.Save(ctx, *testutil.Must(user.Create(username, "a")))
		require.NoError(t, err)
		assert.Equal(t, username, created.Username)

	})

	t.Run("updates an existing user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		username := "testuser"

		users := NewPgUsers(db)

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

func TestPgUsers_ByID(t *testing.T) {
	t.Run("returns the correct user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)

		u := testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByID(ctx, u.ID)

		require.NoError(t, err)
		assert.Equal(t, u.ID, found.ID)
		assert.Equal(t, u.Username, found.Username)
	})

	t.Run("does not return a different user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)
		id := testutil.Must(user.NewID())

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByID(ctx, id)

		require.NoError(t, err)
		assert.Nil(t, found)
	})
}

func TestPgUsers_ByUsername(t *testing.T) {
	t.Run("returns the correct user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)
		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create(username, "a"))))

		found, err := users.ByUsername(ctx, username)

		require.NoError(t, err)
		assert.Equal(t, username, found.Username)
	})

	t.Run("does not return a different user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)
		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByUsername(ctx, username)

		require.NoError(t, err)
		assert.Nil(t, found)
	})
}
