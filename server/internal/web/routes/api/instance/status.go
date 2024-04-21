package instance

import (
	"net/http"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

type statusResponse struct {
	Status status `json:"status"`
}

type status struct {
	PublicURL      string `json:"publicURL"`
	Database       string `json:"database"`
	FileStorage    string `json:"fileStorage"`
	SetupCompleted bool   `json:"setupCompleted"`
}

func from(s instance.Status) status {
	return status{
		PublicURL:      s.PublicURL.String(),
		Database:       string(s.Database),
		FileStorage:    string(s.FileStorage),
		SetupCompleted: s.FirstTimeSetupCompleted,
	}
}

func Status(logger zerolog.Logger, app *core.Application) http.Handler {
	return handler.Handler(logger, func(logger zerolog.Logger, w http.ResponseWriter, r *http.Request) {
		status, err := app.Instance().Status(r.Context())
		if err != nil {
			handler.RespondWithJson(w, http.StatusBadRequest, handler.ErrUnknown(err))
			return
		}

		handler.RespondWithJson(w, http.StatusOK, statusResponse{
			Status: from(status),
		})
	})
}
