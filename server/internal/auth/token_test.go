package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

type testCase struct {
	name  string
	token string
}

func TestTokenVerifier(t *testing.T) {
	t.Run("verifies a valid token", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		tokens := NewTokensMock(t)

		u := testutil.Must(user.Create("test", "test"))
		token := testutil.Must(TokenFor(u))

		tokens.On("ByID", token.Value).Return(token, nil).Once()

		verifier := NewTokenVerifier(tokens)

		err := verifier.VerifyToken(ctx, token.Value)
		assert.NoError(t, err)
	})

	for _, tc := range []testCase{
		{name: "refuses an empty string", token: ""},
		{name: "refuses something that is not a token", token: "thisisnotatoken"},
	} {
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel()

			ctx := context.Background()

			tokens := NewTokensMock(t)

			verifier := NewTokenVerifier(tokens)

			err := verifier.VerifyToken(ctx, TokenID(tc.token))
			assert.ErrorIs(t, err, ErrInvalidToken)
		})
	}

	t.Run("refuses an expired token", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		tokens := NewTokensMock(t)

		u := testutil.Must(user.Create("test", "test"))
		token := testutil.Must(TokenFor(u))
		token.ExpiresAt = time.Now().Add(-1 * time.Hour)

		tokens.On("ByID", token.Value).Return(token, nil).Once()

		verifier := NewTokenVerifier(tokens)

		err := verifier.VerifyToken(ctx, token.Value)
		assert.ErrorIs(t, err, ErrInvalidToken)
	})
}
