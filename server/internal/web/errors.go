package web

import (
	"encoding/json"
	"fmt"
	"net/http"

	nethttpmiddleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type errorFunc func(http.ResponseWriter, *http.Request, error)

func handleApiRequestError(l zerolog.Logger) errorFunc {
	return func(w http.ResponseWriter, r *http.Request, err error) {
		l = l.With().
			Err(err).
			Str("method", r.Method).
			Str("url", r.URL.String()).
			Str("lifecycle", "request").
			Logger()
		handleApiError(l)(w, err.Error(), http.StatusBadRequest)
	}
}

func handleApiResponseError(l zerolog.Logger) errorFunc {
	return func(w http.ResponseWriter, r *http.Request, err error) {
		l = l.With().
			Err(err).
			Str("method", r.Method).
			Str("url", r.URL.String()).
			Str("lifecycle", "response").
			Logger()
		handleApiError(l)(w, err.Error(), http.StatusInternalServerError)
	}
}

func handleApiError(l zerolog.Logger) nethttpmiddleware.ErrorHandler {
	return func(w http.ResponseWriter, message string, status int) {
		e := l.Debug()
		if status >= 500 {
			e = l.Error()
		}

		e.Str("error", message).Int("status", status).Msg("api error")

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(status)

		if err := json.NewEncoder(w).Encode(api.Error{
			Error:   "internal_error",
			Message: message,
		}); err != nil {
			http.Error(w, fmt.Errorf("failed to serialize error response to json: %w", err).Error(), http.StatusInternalServerError)
		}
	}
}
