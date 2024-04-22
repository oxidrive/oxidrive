package user

import (
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func TestUser(t *testing.T) {
	t.Run("can be created with valid parameters", func(t *testing.T) {
		t.Parallel()
		username := "test"
		password := "testpassword"

		created, err := Create(username, password)

		assert.NoError(t, err)
		assert.NotEqual(t, UserID(uuid.Nil), created.ID)
		assert.Equal(t, username, created.Username)
	})

	t.Run("verifies a valid password", func(t *testing.T) {
		t.Parallel()
		username := "test"
		password := "testpassword"

		user, err := Create(username, password)

		assert.NoError(t, err)

		valid, err := user.VerifyPassword(password)
		assert.NoError(t, err)
		assert.True(t, valid)
	})

	t.Run("does not verify an invalid password", func(t *testing.T) {
		t.Parallel()
		username := "test"
		password := "testpassword"

		user, err := Create(username, password)

		assert.NoError(t, err)

		valid, err := user.VerifyPassword("invalid password")
		assert.NoError(t, err)
		assert.False(t, valid)
	})
}
