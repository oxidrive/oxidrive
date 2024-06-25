package web

import (
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"

	"github.com/oxidrive/oxidrive/server/internal/app"
	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
)

func serveBlob(app *app.Application) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		forceDwl, err := forceDownload(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		f, c, err := downloadBlob(app, r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		if f == nil {
			http.NotFound(w, r)
			return
		}

		defer file.Close(c)

		w.Header().Add("content-type", string(f.ContentType))
		w.Header().Add("content-length", strconv.FormatInt(int64(f.Size), 10))
		w.Header().Add("cache-control", "private")
		w.Header().Add("content-disposition", contentDisposition(f, forceDwl))

		if _, err = io.Copy(w, c); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	}
}

func downloadBlob(app *app.Application, r *http.Request) (*file.File, file.Content, error) {
	ctx := r.Context()
	path := r.PathValue("path")

	owner := auth.GetCurrentUser(ctx)

	p, err := file.ParsePath(path)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to parse path from %s: %w", path, err)
	}

	f, err := app.Files().ByOwnerByPath(ctx, owner.ID, p)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to load file by path %s: %w", p, err)
	}

	if f == nil || f.Type == file.TypeFolder {
		return nil, nil, nil
	}

	c, err := app.Files().Download(ctx, *f)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to download file content for file %s: %w", f.ID, err)
	}

	return f, c, nil
}

func forceDownload(r *http.Request) (bool, error) {
	q := r.URL.Query()
	if !q.Has("download") {
		return false, nil
	}

	fd, err := strconv.ParseBool(q.Get("download"))
	if err != nil {
		return false, err
	}

	return fd, nil
}

func contentDisposition(f *file.File, force bool) string {
	if force {
		return attachment(f)
	}

	if canBeInlined(f) {
		return "inline"
	}

	return attachment(f)
}

func canBeInlined(f *file.File) bool {
	for _, ct := range []string{"application/pdf", "image/", "video/", "audio/"} {
		if strings.HasPrefix(string(f.ContentType), ct) {
			return true
		}
	}

	return false
}

func attachment(f *file.File) string {
	return "attachment; filename=" + string(f.Name)
}
