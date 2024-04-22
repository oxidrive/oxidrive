package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestPgUsers_Count(t *testing.T) {
	t.Run("returns the number of users", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		users := NewPgUsers(db)

		testutil.Must(users.Save(ctx, testutil.Must(user.Create("a", "a"))))
		testutil.Must(users.Save(ctx, testutil.Must(user.Create("b", "b"))))

		count, err := users.Count(ctx)

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

		created, err := users.Save(ctx, testutil.Must(user.Create(username, "a")))
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

		created, err := users.Save(ctx, testutil.Must(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)

		changedUsername := "changed"
		created.Username = changedUsername

		updated, err := users.Save(ctx, created)
		assert.NoError(t, err)
		assert.Equal(t, created.ID, updated.ID)
		assert.Equal(t, changedUsername, updated.Username)

	})
}
