package middleware

import (
	"fmt"
	"net/http"
	"strings"
)

func EnforceContentType(contentType string) Middleware {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			ct := r.Header.Get("Content-Type")
			if ct != "" {
				mediaType := strings.ToLower(strings.TrimSpace(strings.Split(ct, ";")[0]))
				if mediaType != contentType {
					msg := fmt.Sprintf("Content-Type header is not %s", contentType)
					http.Error(w, msg, http.StatusUnsupportedMediaType)
					return
				}
			}

			next.ServeHTTP(w, r)
		})
	}
}
