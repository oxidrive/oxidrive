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

func TestTokenService_Verify(t *testing.T) {
	ttl := 1 * time.Hour

	t.Run("verifies a valid token", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		tokens := NewTokensMock(t)
		defer tokens.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		token := testutil.Must(NewToken(u, time.Now().Add(ttl)))

		tokens.On("ByID", token.Value).Return(token, nil).Once()

		svc := NewTokenService(tokens, ttl)

		err := svc.Verify(ctx, token.Value)
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
			defer tokens.AssertExpectations(t)

			svc := NewTokenService(tokens, ttl)

			err := svc.Verify(ctx, TokenID(tc.token))
			assert.ErrorIs(t, err, ErrInvalidToken)
		})
	}

	t.Run("refuses an expired token", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		tokens := NewTokensMock(t)
		defer tokens.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		token := testutil.Must(NewToken(u, time.Now().Add(ttl)))
		token.ExpiresAt = time.Now().Add(-1 * time.Hour)

		tokens.On("ByID", token.Value).Return(token, nil).Once()

		svc := NewTokenService(tokens, ttl)

		err := svc.Verify(ctx, token.Value)
		assert.ErrorIs(t, err, ErrInvalidToken)
	})

	t.Run("refuses a token that does not exist", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		tokens := NewTokensMock(t)
		defer tokens.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		token := testutil.Must(NewToken(u, time.Now().Add(ttl)))

		tokens.On("ByID", token.Value).Return((*Token)(nil), nil).Once()

		svc := NewTokenService(tokens, ttl)

		err := svc.Verify(ctx, token.Value)
		assert.ErrorIs(t, err, ErrInvalidToken)
	})
}
