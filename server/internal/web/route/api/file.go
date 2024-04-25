package api

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

type uploadResponse struct {
	Ok bool `json:"ok"`
}

func Upload(logger zerolog.Logger, app *core.Application) http.Handler {
	return handler.MultipartHandler(logger, func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, req handler.MultipartRequest) {
		ctx := r.Context()

		if err := app.File().Upload(ctx, file.Content(req.File), file.Path(req.FileHeader.Filename), file.Size(req.FileHeader.Size), logger); err != nil {
			handler.RespondWithJson(w, http.StatusInternalServerError, handler.ErrorResponse{
				Error:   "upload_failed",
				Message: err.Error(),
			})
		}

		handler.RespondWithJson(w, http.StatusOK, uploadResponse{
			Ok: true,
		})
	})
}
