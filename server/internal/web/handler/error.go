package handler

import (
	"encoding/json"
	"net/http"
)

type ErrorResponse struct {
	Error   string
	Message string
	Details map[string]interface{}
}

func RespondWithError(w http.ResponseWriter, status int, err ErrorResponse) {
	w.Header().Add("content-type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(err); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func ErrUnknown(err error) ErrorResponse {
	return ErrorResponse{
		Error:   "unknown",
		Message: err.Error(),
	}
}
