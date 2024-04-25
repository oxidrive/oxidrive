package handler

import (
	"errors"
	"net/http"

	"github.com/rs/zerolog"
)

func MultipartHandler(logger zerolog.Logger, h func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, body MultipartRequest)) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var body MultipartRequest

		err := DecodeMutipart(w, r, &body, logger)
		if err != nil {
			var mr *MalformedRequest
			if errors.As(err, &mr) {
				http.Error(w, mr.Msg, mr.Status)
			} else {
				logger.Error().AnErr("error", err).Msg("failed to decode setup request body from mutlipart")
				RespondWithJson(w, http.StatusInternalServerError, ErrUnknown(err))
			}
		}

		l := logger.With().Logger()

		h(l, w, r, body)
	})
}