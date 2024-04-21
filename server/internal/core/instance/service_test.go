package instance

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/stretchr/testify/assert"
)

func TestInstanceService(t *testing.T) {
	t.Run("completes the first time setup", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersSpy()
		svc := NewService(users)

		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		completed, err := svc.FirstTimeSetupCompleted(ctx)
		assert.NoError(t, err)
		assert.False(t, completed)

		err = svc.CompleteFirstTimeSetup(ctx, initial)
		created := users.Users()[0]
		assert.NoError(t, err)
		assert.Equal(t, initial.Username, created.Username)

		valid, err := created.VerifyPassword(initial.Password)
		assert.NoError(t, err)
		assert.True(t, valid)
	})

	t.Run("stops the first time setup flow if a user has already been created", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		existing, err := user.Create("test", "test")
		assert.NoError(t, err)

		users := user.NewUsersSpy(existing)
		svc := NewService(users)

		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		completed, err := svc.FirstTimeSetupCompleted(ctx)
		assert.NoError(t, err)
		assert.True(t, completed)

		err = svc.CompleteFirstTimeSetup(ctx, initial)
		assert.ErrorIs(t, err, ErrSetupAlreadyCompleted)
		assert.Len(t, users.Users(), 1)
	})
}
