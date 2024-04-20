package user

import (
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestPgUsers_Count(t *testing.T) {
	testutil.IntegrationTest(t)

	t.Run("returns the number of users", func(t *testing.T) {})
}

func TestPgUsers_Save(t *testing.T) {
	testutil.IntegrationTest(t)

	t.Run("creates a new user", func(t *testing.T) {})

	t.Run("updates an existing user", func(t *testing.T) {})
}
