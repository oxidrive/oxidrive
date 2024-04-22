package middleware

import (
	"context"
	"net/http"
)

func Authenticate() Middleware {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			username, password, ok := r.BasicAuth()

			if !ok {
				http.Error(w, "The request is missing username and password", http.StatusUnauthorized)
				return
			}

			ctx := r.Context()
			ctx = context.WithValue(ctx, "username", username)
			ctx = context.WithValue(ctx, "password", password)
			r = r.WithContext(ctx)

			next.ServeHTTP(w, r)
		})

	}
}
