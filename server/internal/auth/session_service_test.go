package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

type testCase struct {
	name    string
	session string
}

func TestSessionService_Verify(t *testing.T) {
	ttl := 1 * time.Hour

	t.Run("verifies a valid session", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		sessions := NewSessionsMock(t)
		defer sessions.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		session := testutil.Must(NewSession(u, time.Now().Add(ttl)))

		sessions.On("ByID", session.Value).Return(session, nil).Once()

		svc := NewSessionService(sessions, ttl)

		err := svc.Verify(ctx, session.Value)
		assert.NoError(t, err)
	})

	for _, tc := range []testCase{
		{name: "refuses an empty string", session: ""},
		{name: "refuses something that is not a session", session: "thisisnotasession"},
	} {
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel()

			ctx := context.Background()

			sessions := NewSessionsMock(t)
			sessions.On("ByID", mock.AnythingOfType("auth.SessionID")).Return((*Session)(nil), nil).Once()

			defer sessions.AssertExpectations(t)

			svc := NewSessionService(sessions, ttl)

			err := svc.Verify(ctx, SessionID(tc.session))
			assert.ErrorIs(t, err, ErrInvalidSession)
		})
	}

	t.Run("refuses an expired session", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		sessions := NewSessionsMock(t)
		defer sessions.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		session := testutil.Must(NewSession(u, time.Now().Add(ttl)))
		session.ExpiresAt = time.Now().Add(-1 * time.Hour)

		sessions.On("ByID", session.Value).Return(session, nil).Once()

		svc := NewSessionService(sessions, ttl)

		err := svc.Verify(ctx, session.Value)
		assert.ErrorIs(t, err, ErrInvalidSession)
	})

	t.Run("refuses a session that does not exist", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		sessions := NewSessionsMock(t)
		defer sessions.AssertExpectations(t)

		u := testutil.Must(user.Create("test", "test"))
		session := testutil.Must(NewSession(u, time.Now().Add(ttl)))

		sessions.On("ByID", session.Value).Return((*Session)(nil), nil).Once()

		svc := NewSessionService(sessions, ttl)

		err := svc.Verify(ctx, session.Value)
		assert.ErrorIs(t, err, ErrInvalidSession)
	})
}
