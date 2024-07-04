package auth

import (
	"context"
	"errors"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var (
	ErrAuthenticationFailed = errors.New("authentication failed")
)

type Authenticator struct {
	users    user.Users
	sessions SessionService
}

func NewAuthenticator(users user.Users, sessions SessionService) Authenticator {
	return Authenticator{users: users, sessions: sessions}
}

func (a *Authenticator) AuthenticateWithPassword(ctx context.Context, username string, password string) (*Session, *user.User, error) {
	u, err := a.users.ByUsername(ctx, username)
	if err != nil {
		return nil, nil, err
	}

	if u == nil {
		return nil, nil, ErrAuthenticationFailed
	}

	valid, err := u.VerifyPassword(password)
	if err != nil {
		return nil, nil, err
	}

	if !valid {
		return nil, nil, ErrAuthenticationFailed
	}

	session, err := a.sessions.Generate(ctx, u)
	if err != nil {
		return nil, nil, err
	}

	session, err = a.sessions.Store(ctx, *session)
	if err != nil {
		return nil, nil, err
	}

	return session, u, nil
}

func (a *Authenticator) UserFromSession(ctx context.Context, sid SessionID) (*user.User, error) {
	session, err := a.sessions.ByID(ctx, sid)
	if err != nil {
		return nil, err
	}

	if session == nil {
		return nil, nil
	}

	return a.users.ByID(ctx, session.UserID)
}
