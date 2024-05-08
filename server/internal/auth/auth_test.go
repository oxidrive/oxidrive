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

var ttl = 1 * time.Hour

func TestAuthenticator(t *testing.T) {
	t.Run("authenticates a user with password", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		tokens := NewTokensMock(t)
		svc := NewTokenService(tokens, ttl)

		password := "test"
		u := testutil.Must(user.Create("test", password))
		tk := testutil.Must(svc.Generate(ctx, u))

		a := NewAuthenticator(users, svc)

		users.On("ByUsername", u.Username).Return(u, nil).Once()
		tokens.On("Store", mock.Anything).Return(tk, nil).Once()

		tk, ua, err := a.AuthenticateWithPassword(ctx, u.Username, password)

		assert.Equal(t, tk.UserID, ua.ID)
		assert.Equal(t, u.ID, ua.ID)
		assert.Equal(t, u.Username, ua.Username)
		assert.NoError(t, err)
	})

	t.Run("refuses a user that doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		tokens := NewTokensMock(t)
		svc := NewTokenService(tokens, ttl)

		password := "test"
		u := testutil.Must(user.Create("test", password))

		a := NewAuthenticator(users, svc)

		users.On("ByUsername", mock.Anything).Return((*user.User)(nil), nil).Once()
		tokens.On("Store", mock.Anything).Return((*Token)(nil), nil).Once()

		tk, ua, err := a.AuthenticateWithPassword(ctx, u.Username, password)

		assert.Nil(t, tk)
		assert.Nil(t, ua)
		assert.ErrorIs(t, err, ErrAuthenticationFailed)
	})

	t.Run("refuses an invalid password", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		tokens := NewTokensMock(t)
		svc := NewTokenService(tokens, ttl)

		u := testutil.Must(user.Create("test", "test"))

		a := NewAuthenticator(users, svc)

		users.On("ByUsername", u.Username).Return(u, nil).Once()
		tokens.On("Store", mock.Anything).Return((*Token)(nil), nil).Once()

		tk, ua, err := a.AuthenticateWithPassword(ctx, u.Username, "wrong password")

		assert.Nil(t, tk)
		assert.Nil(t, ua)
		assert.ErrorIs(t, err, ErrAuthenticationFailed)
	})
}

func TestAuthenticator_UserForToken(t *testing.T) {
	t.Run("returns the user associated with the token", func(t *testing.T) {
		t.Parallel()

		ctx := context.Background()

		users := user.NewUsersMock(t)
		tokens := NewTokensMock(t)
		svc := NewTokenService(tokens, ttl)

		u := testutil.Must(user.Create("test", "test"))
		tk := testutil.Must(svc.Generate(ctx, u))

		a := NewAuthenticator(users, svc)

		users.On("ByID", u.ID).Return(u, nil).Once()
		tokens.On("ByID", tk.Value).Return(tk, nil).Once()

		ua, err := a.UserForToken(ctx, tk.Value)

		assert.NoError(t, err)
		assert.Equal(t, u.ID, ua.ID)
	})
}
