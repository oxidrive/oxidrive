package handler

import (
	"context"
	"fmt"
	"io"

	"github.com/gabriel-vasile/mimetype"
	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

type Files struct {
	Logger             zerolog.Logger
	App                *app.Application
	MultipartMaxMemory int64
}

func (f *Files) List(ctx context.Context, request api.FilesListRequestObject) (api.FilesListResponseObject, error) {
	var prefix *file.Path
	if request.Params.Prefix != nil {
		pfx := string(*request.Params.Prefix)
		p, err := file.ParsePath(pfx)
		if err != nil {
			return nil, err
		}

		prefix = &p
	}

	ff, err := f.App.Files().List(ctx, prefix, list.First(request.Params.First), list.After(list.CursorFromString(request.Params.After)))
	if err != nil {
		return nil, err
	}

	count := len(ff.Items)

	items := make([]api.File, count)
	for i, fi := range ff.Items {

		items[i] = ToApiFile(fi)
	}

	return api.FilesList200JSONResponse(api.FileList{
		Count: count,
		Items: items,
		Next:  ff.Next.ToString(),
		Total: ff.Total,
	}), nil
}

func (f *Files) Upload(ctx context.Context, request api.FilesUploadRequestObject) (api.FilesUploadResponseObject, error) {
	owner := auth.GetCurrentUser(ctx)

	form, err := request.Body.ReadForm(f.MultipartMaxMemory)
	if err != nil {
		return nil, fmt.Errorf("failed to read form data from request body: %w", err)
	}

	paths := form.Value["path"]
	if len(paths) == 0 {
		return api.FilesUpload400JSONResponse{ErrorJSONResponse: api.ErrorJSONResponse(api.Error{
			Error:   "missing_path",
			Message: "form is missing required parameter 'path'",
		})}, nil
	}

	p := paths[0]

	files := form.File["file"]
	if len(files) == 0 {
		return api.FilesUpload400JSONResponse{ErrorJSONResponse: api.ErrorJSONResponse(api.Error{
			Error:   "missing_file",
			Message: "form is missing required parameter 'file'",
		})}, nil
	}

	fh := files[0]
	ff, err := fh.Open()
	if err != nil {
		return nil, err
	}

	ct, err := mimetype.DetectReader(ff)
	if err != nil {
		return nil, err
	}

	// Rewind the file as mimetype.DetectReader consumes the first bytes
	// to detect the content type
	if _, err = ff.Seek(0, io.SeekStart); err != nil {
		return nil, err
	}

	path, err := file.ParsePath(p)
	if err != nil {
		return nil, fmt.Errorf("failed to parse uploaded file path %s: %w", p, err)
	}

	upload := file.FileUpload{
		Content:     file.Content(ff),
		ContentType: file.ContentType(ct.String()),
		Path:        path,
		Size:        file.Size(fh.Size),
	}

	id, err := f.App.Files().Upload(ctx, upload, owner.ID)
	if err != nil {
		return nil, err
	}

	return api.FilesUpload200JSONResponse(api.FileUploadResponse{
		Ok: true,
		Id: id.String(),
	}), nil
}

func (f *Files) Delete(ctx context.Context, request api.FileDeleteRequestObject) (api.FileDeleteResponseObject, error) {
	cu := auth.GetCurrentUser(ctx)
	id := file.ID(request.Id)

	fi, err := f.App.Files().ByID(ctx, id)
	if err != nil {
		return nil, fmt.Errorf("failed to load file %s: %w", id, err)
	}

	// TODO: the owner check will eventually be performed by the authorization engine
	if fi == nil || fi.OwnerID != cu.ID {
		return api.FileDelete404JSONResponse{
			NotFoundJSONResponse: api.NotFoundJSONResponse{
				Error:   api.NotFoundErrorErrorNotFound,
				Message: fmt.Sprintf("file %s not found", id),
			},
		}, nil
	}

	if err = f.App.Files().Delete(ctx, fi.ID); err != nil {
		return nil, fmt.Errorf("failed to delete file %s: %w", id, err)
	}

	return api.FileDelete200JSONResponse(ToApiFile(*fi)), nil
}

func ToApiFile(f file.File) api.File {
	var typ api.FileType

	switch f.Type {
	case file.TypeFile:
		typ = api.FileTypeFile
	case file.TypeFolder:
		typ = api.FileTypeFolder
	}

	return api.File{
		Id:          f.ID.AsUUID(),
		Type:        typ,
		ContentType: string(f.ContentType),
		Name:        string(f.Name),
		Path:        string(f.Path),
		Size:        int(f.Size),
	}

}
