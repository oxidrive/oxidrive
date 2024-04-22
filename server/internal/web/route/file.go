package route

import (
	"net/http"
	"path/filepath"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
)

func FilesHandler(fileService file.Service, logger zerolog.Logger) http.HandlerFunc {
	return func(w http.ResponseWriter, req *http.Request) {
		switch req.Method {
		case http.MethodPut:
			defer req.Body.Close()
			if err := req.ParseMultipartForm(0); err != nil {
				http.Error(w, err.Error(), http.StatusBadRequest)
				return
			}
			defer func() {
				if err := req.MultipartForm.RemoveAll(); err != nil {
					logger.Error().AnErr("error", err).Msg("error while removing temporary files during file upload")
				}
			}()

			uploadedFile, uploadedFileHeader, err := req.FormFile("file")
			if err != nil {
				if err == http.ErrMissingFile {
					http.Error(w, "Request did not contain a file", http.StatusBadRequest)
				} else {
					http.Error(w, err.Error(), http.StatusBadRequest)
				}
				return
			}

			defer func() {
				if err := uploadedFile.Close(); err != nil {
					logger.Error().AnErr("error", err).Msg("error while closing uploaded file during file upload")
				}
			}()

			if err := fileService.Upload(
				req.Context(),
				file.Content(uploadedFile),
				file.Name(filepath.Base(uploadedFileHeader.Filename)),
				file.Path((uploadedFileHeader.Filename)),
				file.Size(uploadedFileHeader.Size),
				logger,
			); err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
		default:
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
	}
}
