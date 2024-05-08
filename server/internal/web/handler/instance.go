package handler

import (
	"context"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/core/instance"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type Instance struct {
	Logger zerolog.Logger
	App    *app.Application
}

func (i Instance) Setup(ctx context.Context, request api.InstanceSetupRequestObject) (api.InstanceSetupResponseObject, error) {
	completed, err := i.App.Instance().FirstTimeSetupCompleted(ctx)
	if err != nil {
		return nil, err
	}

	if completed {
		return api.InstanceSetup409JSONResponse(api.Error{
			Error:   "setup_already_completed",
			Message: "first time setup has already been completed",
		}), nil
	}

	if err := i.App.Instance().CompleteFirstTimeSetup(ctx, instance.InitialAdmin{
		Username: request.Body.Admin.Username,
		Password: request.Body.Admin.Password,
	}); err != nil {
		return api.InstanceSetup400JSONResponse{ErrorJSONResponse: api.ErrorJSONResponse(api.Error{
			Error:   "setup_failed",
			Message: err.Error(),
		})}, nil
	}

	return api.InstanceSetup200JSONResponse(api.InstanceSetupResponse{
		Ok: true,
	}), nil
}

func (i Instance) Status(ctx context.Context, request api.InstanceStatusRequestObject) (api.InstanceStatusResponseObject, error) {
	status, err := i.App.Instance().Status(ctx)
	if err != nil {
		return nil, err
	}

	return api.InstanceStatus200JSONResponse(api.InstanceStatus{
		Status: struct {
			Database       api.InstanceStatusStatusDatabase    `json:"database"`
			FileStorage    api.InstanceStatusStatusFileStorage `json:"fileStorage"`
			PublicURL      string                              `json:"publicURL"`
			SetupCompleted bool                                `json:"setupCompleted"`
		}{
			Database:       api.InstanceStatusStatusDatabase(status.Database),
			FileStorage:    api.InstanceStatusStatusFileStorage(status.FileStorage),
			PublicURL:      status.PublicURL.String(),
			SetupCompleted: status.FirstTimeSetupCompleted,
		},
	}), nil
}
