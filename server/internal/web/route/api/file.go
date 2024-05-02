package api

import (
	"net/http"

	"github.com/google/uuid"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
	"github.com/oxidrive/oxidrive/server/internal/web/middleware"
)

type uploadResponse struct {
	Ok bool `json:"ok"`
}

func Upload(logger zerolog.Logger, app *core.Application, cfg handler.Config) http.Handler {
	return handler.MultipartHandler(logger, cfg.MultipartMaxMemory, func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, req handler.MultipartRequest) {
		ctx := r.Context()
		defer req.CloseFunc()

		authToken, ok := ctx.Value(middleware.AuthToken{}).(string)
		if !ok || authToken == "" {
			handler.RespondWithJson(w, http.StatusUnauthorized, handler.ErrorResponse{
				Error:   "upload_failed",
				Message: "The request is missing the authorization token",
			})
			return
		}

		userID, err := uuid.Parse(getUserIDFromAuthToken(authToken)) // TODO delete this call after creating the user service
		if err != nil {
			handler.RespondWithJson(w, http.StatusUnauthorized, handler.ErrorResponse{
				Error:   "upload_failed",
				Message: err.Error(),
			})
			return
		}

		toUpload := file.FileUpload{Content: file.Content(req.File), Path: file.Path(req.FileHeader.Filename), Size: file.Size(req.FileHeader.Size)}

		if err := app.Files().Upload(ctx, toUpload, user.ID(userID)); err != nil {
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

// TODO delete this function after creating the user service
func getUserIDFromAuthToken(authToken string) string {
	return authToken
}
