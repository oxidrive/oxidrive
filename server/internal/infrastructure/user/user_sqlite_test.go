package user

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestSqliteUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		users := NewSqliteUsers(db)

		testutil.Must(users.Save(testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(testutil.Must(user.Create("b", "b"))))

		count, err := users.Count()

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

		created, err := users.Save(testutil.Must(user.Create(username, "a")))
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

		created, err := users.Save(testutil.Must(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)

		changedUsername := "changed"
		created.Username = changedUsername

		updated, err := users.Save(created)
		assert.NoError(t, err)
		assert.Equal(t, created.Id, updated.Id)
		assert.Equal(t, changedUsername, updated.Username)
	})
}
