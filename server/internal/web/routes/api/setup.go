package api

import (
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
	"github.com/rs/zerolog"
)

type setupRequest struct {
	Admin initialAdminData
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
	return handler.JsonHandler[setupRequest](logger, func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request, req setupRequest) {
		completed, err := app.Instance().FirstTimeSetupCompleted()
		if err != nil {
			handler.RespondWithError(w, http.StatusBadRequest, handler.ErrorResponse{
				Error:   "setup_failed",
				Message: err.Error(),
			})
			return
		}

		if completed {
			handler.RespondWithError(w, http.StatusConflict, handler.ErrorResponse{
				Error:   "setup_already_completed",
				Message: "first time setup has already been completed",
			})
			return
		}

		if err := app.Instance().CompleteFirstTimeSetup(req.Admin.into()); err != nil {
			handler.RespondWithError(w, http.StatusBadRequest, handler.ErrorResponse{
				Error:   "setup_failed",
				Message: err.Error(),
			})
			return
		}
	})
}
