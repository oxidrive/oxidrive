package auth

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestPgTokens(t *testing.T) {
	t.Run("stores and returns a token by ID", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		tokens := NewPgTokens(db)

		u := testutil.Must(user.Create("a", "a"))
		t1 := testutil.Must(tokens.Store(ctx, *testutil.Must(auth.TokenFor(u))))
		testutil.Must(tokens.Store(ctx, *testutil.Must(auth.TokenFor(u))))

		found, err := tokens.ByID(ctx, t1.Value)

		assert.NoError(t, err)
		assert.Equal(t, t1.Value, found.Value)
	})
}
