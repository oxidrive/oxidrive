package instance

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestInstanceService(t *testing.T) {
	t.Run("completes the first time setup", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		created := testutil.Must(user.Create(initial.Username, initial.Password))
		users := user.NewUsersMock(t)
		users.On("Count", ctx).Return(0, nil).Twice()
		users.On("Save", ctx, mock.MatchedBy(func(u user.User) bool { return u.Username == created.Username })).Return((*user.User)(nil), nil).Once()

		svc := NewService(users)

		completed, err := svc.FirstTimeSetupCompleted(ctx)
		require.NoError(t, err)
		assert.False(t, completed)

		err = svc.CompleteFirstTimeSetup(ctx, initial)
		require.NoError(t, err)
	})

	t.Run("stops the first time setup flow if a user has already been created", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		users.On("Count", ctx).Return(1, nil).Twice()

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
	})
}
