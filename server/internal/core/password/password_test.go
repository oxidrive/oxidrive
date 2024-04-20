package password

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPasswordHash(t *testing.T) {
	t.Run("accepts a matching password", func(t *testing.T) {
		t.Parallel()

		password := "testpassword"

		hash, err := ValidateAndHash(password)

		assert.NoError(t, err)

		valid, err := hash.Verify(password)
		assert.NoError(t, err)
		assert.True(t, valid)
	})

	t.Run("rejects a wrong password", func(t *testing.T) {
		t.Parallel()

		password := "testpassword"

		hash, err := ValidateAndHash(password)

		assert.NoError(t, err)

		valid, err := hash.Verify("invalidpassword")
		assert.NoError(t, err)
		assert.False(t, valid)
	})

	t.Run("exposes the hash as a string", func(t *testing.T) {
		t.Parallel()

		password := "testpassword"

		hash, err := ValidateAndHash(password)

		assert.NoError(t, err)

		s := hash.Expose()
		assert.NotEmpty(t, s)
	})
}
