package auth

import (
	"context"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type ctxCurrentUser struct{}

func WithCurrentUser(parent context.Context, u *user.User) context.Context {
	return context.WithValue(parent, ctxCurrentUser{}, u)
}

func GetCurrentUser(ctx context.Context) *user.User {
	u, _ := ctx.Value(ctxCurrentUser{}).(*user.User)
	return u
}
