package web

import (
	"context"
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/api"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

var _ api.StrictServerInterface = (*Api)(nil)

type Api struct {
	files    handler.Files
	instance handler.Instance
}

func mountApi(router *http.ServeMux, cfg *Config) error {
	a := &Api{
		files: handler.Files{
			Logger: cfg.Logger.With().Str("handler", "files").Logger(),
			App:    cfg.Application,
		},
		instance: handler.Instance{
			Logger: cfg.Logger.With().Str("handler", "instance").Logger(),
			App:    cfg.Application,
		},
	}

	middlewares, err := defaultMiddlewares(cfg.Logger)
	if err != nil {
		return err
	}

	api.HandlerWithOptions(api.NewStrictHandler(a, []api.StrictMiddlewareFunc{}), api.StdHTTPServerOptions{
		BaseRouter:  router,
		Middlewares: middlewares,
	})
	mountSwagger(router)

	return nil
}

func (api *Api) InstanceStatus(ctx context.Context, request api.InstanceStatusRequestObject) (api.InstanceStatusResponseObject, error) {
	return api.instance.Status(ctx, request)
}

func (api *Api) InstanceSetup(ctx context.Context, request api.InstanceSetupRequestObject) (api.InstanceSetupResponseObject, error) {
	return api.instance.Setup(ctx, request)
}

func (api *Api) FilesUpload(ctx context.Context, request api.FilesUploadRequestObject) (api.FilesUploadResponseObject, error) {
	return api.files.Upload(ctx, request)
}
