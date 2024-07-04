package web

import (
	"context"
	"net/http"

	"github.com/oxidrive/oxidrive/server/internal/web/api"
	"github.com/oxidrive/oxidrive/server/internal/web/handler"
)

type Api struct {
	auth     handler.Sessions
	files    handler.Files
	instance handler.Instance
}

func mountApi(router *http.ServeMux, cfg *Config) error {
	apis := &Api{
		auth: handler.Sessions{
			Logger: cfg.Logger.With().Str("handler", "auth").Logger(),
			App:    cfg.Application,
		},
		files: handler.Files{
			Logger:             cfg.Logger.With().Str("handler", "files").Logger(),
			App:                cfg.Application,
			MultipartMaxMemory: cfg.MultipartMaxMemory,
		},
		instance: handler.Instance{
			Logger: cfg.Logger.With().Str("handler", "instance").Logger(),
			App:    cfg.Application,
		},
	}

	middlewares, err := defaultMiddlewares(cfg.Logger, cfg.Application)
	if err != nil {
		return err
	}

	handler := api.NewStrictHandlerWithOptions(
		apis,
		[]api.StrictMiddlewareFunc{session(cfg.Application)},
		api.StrictHTTPServerOptions{
			RequestErrorHandlerFunc:  handleApiRequestError(cfg.Logger),
			ResponseErrorHandlerFunc: handleApiResponseError(cfg.Logger),
		},
	)

	api.HandlerWithOptions(handler, api.StdHTTPServerOptions{
		BaseRouter:       router,
		Middlewares:      middlewares,
		ErrorHandlerFunc: handleApiRequestError(cfg.Logger),
	})

	mountSwagger(router)

	return nil
}

var _ api.StrictServerInterface = (*Api)(nil)

/* Auth */

func (api *Api) AuthCreateSession(ctx context.Context, request api.AuthCreateSessionRequestObject) (api.AuthCreateSessionResponseObject, error) {
	return api.auth.CreateSession(ctx, request)
}

func (api *Api) AuthGetSession(ctx context.Context, request api.AuthGetSessionRequestObject) (api.AuthGetSessionResponseObject, error) {
	return api.auth.GetSession(ctx, request)
}

/* Files */

func (api *Api) FilesList(ctx context.Context, request api.FilesListRequestObject) (api.FilesListResponseObject, error) {
	return api.files.List(ctx, request)
}

func (api *Api) FilesUpload(ctx context.Context, request api.FilesUploadRequestObject) (api.FilesUploadResponseObject, error) {
	return api.files.Upload(ctx, request)
}

func (api *Api) FilePatch(ctx context.Context, request api.FilePatchRequestObject) (api.FilePatchResponseObject, error) {
	return api.files.Patch(ctx, request)
}

func (api *Api) FileDelete(ctx context.Context, request api.FileDeleteRequestObject) (api.FileDeleteResponseObject, error) {
	return api.files.Delete(ctx, request)
}

/* Instance */

func (api *Api) InstanceStatus(ctx context.Context, request api.InstanceStatusRequestObject) (api.InstanceStatusResponseObject, error) {
	return api.instance.Status(ctx, request)
}

func (api *Api) InstanceSetup(ctx context.Context, request api.InstanceSetupRequestObject) (api.InstanceSetupResponseObject, error) {
	return api.instance.Setup(ctx, request)
}
