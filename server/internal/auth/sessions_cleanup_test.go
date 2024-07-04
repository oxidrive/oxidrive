package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

func TestSessionsCleanupJob(t *testing.T) {
	t.Run("removes all expired sessions", func(t *testing.T) {
		ctx := context.Background()

		expired := Session{
			Value:     "b",
			UserID:    user.NewID(),
			ExpiresAt: time.Now().Add(-1 * time.Hour),
		}

		sessions := NewSessionsMock(t)
		sessions.On("ExpiringBefore", mock.MatchedBy(isToday)).Return([]Session{expired}, nil).Once()
		sessions.On("DeleteAll", []Session{expired}).Return(nil).Once()
		defer sessions.AssertExpectations(t)

		j := NewSessionsCleanupJob(sessions)

		err := j.Run(ctx)

		require.NoError(t, err)
	})

	t.Run("does nothing if no session is expired", func(t *testing.T) {
		ctx := context.Background()

		sessions := NewSessionsMock(t)
		sessions.On("ExpiringBefore", mock.MatchedBy(isToday)).Return([]Session{}, nil).Once()
		defer sessions.AssertExpectations(t)

		j := NewSessionsCleanupJob(sessions)

		err := j.Run(ctx)

		require.NoError(t, err)
	})
}

func isToday(t time.Time) bool {
	yn, mn, dn := time.Now().Date()
	y, m, d := t.Date()
	return y == yn && m == mn && d == dn
}
