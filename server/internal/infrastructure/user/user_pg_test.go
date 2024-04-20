package user

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/stretchr/testify/assert"
)

func TestPgUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)

		testutil.Must(users.Save(testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(testutil.Must(user.Create("b", "b"))))

		count, err := users.Count()

		assert.NoError(t, err)
		assert.Equal(t, 2, count)

	})
}

func TestPgUsers_Save(t *testing.T) {
	t.Run("creates a new user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		username := "testusername"

		users := NewPgUsers(db)

		created, err := users.Save(testutil.Must(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)

	})

	t.Run("updates an existing user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		username := "testuser"

		users := NewPgUsers(db)

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
