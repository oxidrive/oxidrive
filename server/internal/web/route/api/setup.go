package api

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

type setupRequest struct {
	Admin initialAdminData
}

type setupResponse struct {
	Ok bool `json:"ok"`
}

type initialAdminData struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func (d initialAdminData) into() instance.InitialAdmin {
	return instance.InitialAdmin{
		Username: d.Username,
		Password: d.Password,
	}
}

func Setup(logger zerolog.Logger, app *core.Application) http.Handler {
	return handler.JsonHandler(logger, func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, req setupRequest) {
		ctx := r.Context()
		defer handler.CloseBody(r, logger)

		completed, err := app.Instance().FirstTimeSetupCompleted(ctx)
		if err != nil {
			handler.RespondWithJson(w, http.StatusBadRequest, handler.ErrorResponse{
				Error:   "setup_failed",
				Message: err.Error(),
			})
			return
		}

		if completed {
			handler.RespondWithJson(w, http.StatusConflict, handler.ErrorResponse{
				Error:   "setup_already_completed",
				Message: "first time setup has already been completed",
			})
			return
		}

		if err := app.Instance().CompleteFirstTimeSetup(ctx, req.Admin.into()); err != nil {
			handler.RespondWithJson(w, http.StatusBadRequest, handler.ErrorResponse{
				Error:   "setup_failed",
				Message: err.Error(),
			})
			return
		}

		handler.RespondWithJson(w, http.StatusOK, setupResponse{
			Ok: true,
		})
	})
}
