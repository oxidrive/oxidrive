package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

func TestTokensCleanupJob(t *testing.T) {
	t.Run("removes all expired tokens", func(t *testing.T) {
		ctx := context.Background()

		expired := Token{
			Value:     "b",
			UserID:    user.NewID(),
			ExpiresAt: time.Now().Add(-1 * time.Hour),
		}

		tokens := NewTokensMock(t)
		tokens.On("ExpiringBefore", mock.MatchedBy(isToday)).Return([]Token{expired}, nil).Once()
		tokens.On("DeleteAll", []Token{expired}).Return(nil).Once()
		defer tokens.AssertExpectations(t)

		j := NewTokenCleanupJob(tokens)

		err := j.Run(ctx)

		require.NoError(t, err)
	})

	t.Run("does nothing if no token is expired", func(t *testing.T) {
		ctx := context.Background()

		tokens := NewTokensMock(t)
		tokens.On("ExpiringBefore", mock.MatchedBy(isToday)).Return([]Token{}, nil).Once()
		defer tokens.AssertExpectations(t)

		j := NewTokenCleanupJob(tokens)

		err := j.Run(ctx)

		require.NoError(t, err)
	})
}

func isToday(t time.Time) bool {
	yn, mn, dn := time.Now().Date()
	y, m, d := t.Date()
	return y == yn && m == mn && d == dn
}
