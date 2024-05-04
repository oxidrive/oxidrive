package instance

import (
	"context"
	"net/url"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

var info = Info{
	PublicURL:   testutil.Must(url.Parse("https://example.org")),
	Database:    StatusDBSqlite,
	FileStorage: StatusFileStorageFS,
}

func TestInstanceService_FirstTimeSetup(t *testing.T) {
	t.Run("can be completed if it wasn't before", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		created := testutil.Must(user.Create(initial.Username, initial.Password))
		users := user.NewUsersMock(t)
		users.On("Count").Return(0, nil).Twice()
		users.On("Save", mock.MatchedBy(func(u user.User) bool { return u.Username == created.Username })).Return((*user.User)(nil), nil).Once()

		svc := InitService(info, users)

		completed, err := svc.FirstTimeSetupCompleted(ctx)
		require.NoError(t, err)
		assert.False(t, completed)

		err = svc.CompleteFirstTimeSetup(ctx, initial)
		require.NoError(t, err)
	})

	t.Run("stops if a user has already been created", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		users.On("Count").Return(1, nil).Twice()

		svc := InitService(info, users)

		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		completed, err := svc.FirstTimeSetupCompleted(ctx)
		require.NoError(t, err)
		assert.True(t, completed)

		err = svc.CompleteFirstTimeSetup(ctx, initial)
		assert.ErrorIs(t, err, ErrSetupAlreadyCompleted)
	})
}

func TestInstanceService_Status(t *testing.T) {
	t.Run("returns the current status of the instance", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		expected := Status{
			Info:                    info,
			FirstTimeSetupCompleted: true,
		}

		users := user.NewUsersMock(t)
		users.On("Count").Return(1, nil).Twice()

		svc := InitService(info, users)

		status, err := svc.Status(ctx)
		require.NoError(t, err)
		assert.Equal(t, expected, status)
	})
}
