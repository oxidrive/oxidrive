package handler

import (
	"errors"
	"net/http"

	"github.com/rs/zerolog"
)

func JsonHandler[T any](logger zerolog.Logger, h func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, body T)) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body T

		err := DecodeJSONBody(w, r, &body)
		if err != nil {
			var mr *MalformedRequest
			if errors.As(err, &mr) {
				http.Error(w, mr.Msg, mr.Status)
			} else {
				logger.Error().AnErr("error", err).Msg("failed to decode setup request body from JSON")
				RespondWithJson(w, http.StatusInternalServerError, ErrUnknown(err))
			}
			return
		}

		l := logger.With().Logger()

		h(l, w, r, body)
	})
}
