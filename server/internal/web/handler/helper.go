package handler

import (
	"encoding/json"
	"mime/multipart"
	"net/http"

	"github.com/rs/zerolog"
)

type MultipartRequest struct {
	File       multipart.File
	FileHeader *multipart.FileHeader
	CloseFunc  func()
}

func CloseBody(r *http.Request, logger zerolog.Logger) {
	if err := r.Body.Close(); err != nil {
		logger.Warn().AnErr("error", err).Msg("error while closing body of the request")
	}

}

func RespondWithJson[T any](w http.ResponseWriter, status int, body T) {
	w.Header().Add("content-type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(body); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
