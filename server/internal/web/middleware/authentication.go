package middleware

import (
	"context"
	"net/http"
	"strings"
)

type AuthToken struct{}

func Authenticate() Middleware {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			authorization := r.Header.Get("Authorization")
			if authorization == "" {
				http.Error(w, "The request is missing the authorization token", http.StatusUnauthorized)
				return
			}

			token := strings.TrimSpace(strings.Replace(authorization, "Bearer", "", 1))

			ctx := r.Context()
			ctx = context.WithValue(ctx, AuthToken{}, token)
			r = r.WithContext(ctx)

			next.ServeHTTP(w, r)
		})

	}
}
