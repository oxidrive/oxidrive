package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestSqliteUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		users := NewSqliteUsers(db)

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("b", "b"))))

		count, err := users.Count(ctx)

		assert.NoError(t, err)
		assert.Equal(t, 2, count)
	})
}

func TestSqliteUsers_Save(t *testing.T) {
	t.Run("creates a new user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()
		db := testutil.SqliteDBFromContext(ctx, t)

		username := "testuser"

		users := NewSqliteUsers(db)

		created, err := users.Save(ctx, *testutil.Must(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)
	})

	t.Run("updates an existing user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		username := "testuser"

		users := NewSqliteUsers(db)

		created, err := users.Save(ctx, *testutil.Must(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)

		changedUsername := "changed"
		created.Username = changedUsername

		updated, err := users.Save(ctx, *created)
		assert.NoError(t, err)
		assert.Equal(t, created.ID, updated.ID)
		assert.Equal(t, changedUsername, updated.Username)
	})
}

func TestSqliteUsers_ByUsername(t *testing.T) {
	t.Run("returns the correct user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		users := NewSqliteUsers(db)
		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create(username, "a"))))

		found, err := users.ByUsername(ctx, username)

		assert.NoError(t, err)
		assert.Equal(t, username, found.Username)
	})

	t.Run("does not return a different user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		users := NewSqliteUsers(db)
		username := "test"

		testutil.Must(users.Save(ctx, *testutil.Must(user.Create("a", "a"))))

		found, err := users.ByUsername(ctx, username)

		assert.ErrorIs(t, err, user.ErrUserNotFound)
		assert.Nil(t, found)

	})
}
