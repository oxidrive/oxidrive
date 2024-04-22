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
		defer req.CloseFunc()

		username := ctx.Value("username")
		password := ctx.Value("password")

		if err := app.File().Upload(ctx, file.Content(req.File), file.Path(req.FileHeader.Filename), file.Size(req.FileHeader.Size), logger); err != nil {
			handler.RespondWithJson(w, http.StatusInternalServerError, handler.ErrorResponse{
				Error:   "upload_failed",
				Message: err.Error(),
			})
			return
		}

		handler.RespondWithJson(w, http.StatusOK, uploadResponse{
			Ok: true,
		})
	})
}
