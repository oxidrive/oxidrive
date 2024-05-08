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
	users  user.Users
	tokens TokenService
}

func NewAuthenticator(users user.Users, tokens TokenService) Authenticator {
	return Authenticator{users: users, tokens: tokens}
}

func (a *Authenticator) AuthenticateWithPassword(ctx context.Context, username string, password string) (*Token, *user.User, error) {
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

	t, err := a.tokens.Generate(ctx, u)
	if err != nil {
		return nil, nil, err
	}

	t, err = a.tokens.Store(ctx, *t)
	if err != nil {
		return nil, nil, err
	}

	return t, u, nil
}

func (a *Authenticator) UserForToken(ctx context.Context, token TokenID) (*user.User, error) {
	tk, err := a.tokens.ByID(ctx, token)
	if err != nil {
		return nil, err
	}

	if tk == nil {
		return nil, nil
	}

	return a.users.ByID(ctx, tk.UserID)
}
