package handler

import (
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/rs/zerolog"
)

func DecodeJSONBody(w http.ResponseWriter, r *http.Request, dst interface{}) error {
	r.Body = http.MaxBytesReader(w, r.Body, 1048576)

	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()

	err := dec.Decode(&dst)
	if err != nil {
		var syntaxError *json.SyntaxError
		var unmarshalTypeError *json.UnmarshalTypeError

		switch {
		case errors.As(err, &syntaxError):
			msg := fmt.Sprintf("Request body contains badly-formed JSON (at position %d)", syntaxError.Offset)
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}

		case errors.Is(err, io.ErrUnexpectedEOF):
			msg := "Request body contains badly-formed JSON"
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}

		case errors.As(err, &unmarshalTypeError):
			msg := fmt.Sprintf("Request body contains an invalid value for the %q field (at position %d)", unmarshalTypeError.Field, unmarshalTypeError.Offset)
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}

		case strings.HasPrefix(err.Error(), "json: unknown field "):
			fieldName := strings.TrimPrefix(err.Error(), "json: unknown field ")
			msg := fmt.Sprintf("Request body contains unknown field %s", fieldName)
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}

		case errors.Is(err, io.EOF):
			msg := "Request body must not be empty"
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}

		case err.Error() == "http: request body too large":
			msg := "Request body must not be larger than 1MB"
			return &MalformedRequest{Status: http.StatusRequestEntityTooLarge, Msg: msg}

		default:
			return err
		}
	}

	err = dec.Decode(&struct{}{})
	if !errors.Is(err, io.EOF) {
		msg := "Request body must only contain a single JSON object"
		return &MalformedRequest{Status: http.StatusBadRequest, Msg: msg}
	}

	return nil
}

func DecodeMutipartBody(w http.ResponseWriter, r *http.Request, dst *MultipartRequest, logger zerolog.Logger) error {
	if err := r.ParseMultipartForm(0); err != nil {
		return &MalformedRequest{Status: http.StatusBadRequest, Msg: err.Error()}
	}
	defer func() {
	}()

	uploadedFile, uploadedFileHeader, err := r.FormFile("file")
	if err != nil {
		if err == http.ErrMissingFile {
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: "Request did not contain a file"}

		} else {
			return &MalformedRequest{Status: http.StatusBadRequest, Msg: err.Error()}
		}
	}

	dst.File = uploadedFile
	dst.FileHeader = uploadedFileHeader
	dst.CloseFunc = func() {
		if err := r.MultipartForm.RemoveAll(); err != nil {
			logger.Error().AnErr("error", err).Msg("error while removing temporary files")
		}
		if err := uploadedFile.Close(); err != nil {
			logger.Error().AnErr("error", err).Msg("error while closing uploaded file")
		}
		CloseBody(r, logger)
	}

	return nil
}
