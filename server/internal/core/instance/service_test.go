package instance

import (
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/stretchr/testify/assert"
)

func TestInstanceService(t *testing.T) {
	users := user.NewUsersSpy()
	svc := NewService(users)

	t.Run("completes the first time setup", func(t *testing.T) {
		initial := InitialAdmin{
			Username: "test",
			Password: "testpassword",
		}

		err := svc.CompleteFirstTimeSetup(initial)

		created := users.Users()[0]

		assert.NoError(t, err)
		assert.Equal(t, initial.Username, created.Username)

		valid, err := created.VerifyPassword(initial.Password)
		assert.NoError(t, err)
		assert.True(t, valid)
	})
}
