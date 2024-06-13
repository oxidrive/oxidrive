package web

import (
	"context"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"testing"

	"github.com/go-http-utils/headers"
	"github.com/steinfletcher/apitest"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/oxidrive/oxidrive/server/internal/web/api"
)

func TestApi_Files_List(t *testing.T) {
	t.Run("returns all uploaded files", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		tkn, u, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		body := "hello world!"
		size := len(body)

		id1 := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/hello.txt"),
			Size:        file.Size(size),
		}, u.ID))

		id2 := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/something/else.txt"),
			Size:        file.Size(size),
		}, u.ID))

		var resp api.FileList

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/files").
			WithContext(ctx).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&resp)

		assert.Equal(t, 3, resp.Count)
		assert.Equal(t, 3, resp.Total)
		require.Nil(t, resp.Next)
		assert.Equal(t, len(resp.Items), resp.Count)

		x := resp.Items[0]
		assert.Equal(t, "something", x.Name)
		assert.Equal(t, "/something", resp.Items[0].Path)
		assert.Equal(t, api.FileTypeFolder, resp.Items[0].Type)
		assert.Equal(t, size, resp.Items[0].Size)

		f1 := resp.Items[1]
		assert.Equal(t, id1.AsUUID(), f1.Id)
		assert.Equal(t, api.FileTypeFile, f1.Type)
		assert.Equal(t, "text/plain", f1.ContentType)
		assert.Equal(t, "hello.txt", f1.Name)
		assert.Equal(t, "/hello.txt", f1.Path)
		assert.Equal(t, size, f1.Size)

		f2 := resp.Items[2]
		assert.Equal(t, id2.AsUUID(), f2.Id)
		assert.Equal(t, api.FileTypeFile, f2.Type)
		assert.Equal(t, "text/plain", f2.ContentType)
		assert.Equal(t, "else.txt", f2.Name)
		assert.Equal(t, "/something/else.txt", f2.Path)
		assert.Equal(t, size, f2.Size)
	})

	t.Run("returns uploaded files with a specific prefix", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		tkn, u, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		body := "hello world!"
		path := "/path/to/hello.txt"
		size := len(body)

		id := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path(path),
			Size:        file.Size(size),
		}, u.ID))

		_ = testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader("")),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/path/to/something/else.txt"),
			Size:        file.Size(0),
		}, u.ID))

		var resp api.FileList

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/files").
			Query("prefix", "/path/to").
			WithContext(ctx).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&resp)

		assert.Equal(t, 2, resp.Count)
		assert.Equal(t, 2, resp.Total)
		require.Nil(t, resp.Next)
		assert.Equal(t, len(resp.Items), resp.Count)

		d := resp.Items[0]
		assert.Equal(t, "something", d.Name)
		assert.Equal(t, api.FileTypeFolder, d.Type)
		assert.Equal(t, "/path/to/something", d.Path)
		assert.Equal(t, 0, d.Size)

		f := resp.Items[1]
		assert.Equal(t, id.AsUUID(), f.Id)
		assert.Equal(t, api.FileTypeFile, f.Type)
		assert.Equal(t, "text/plain", f.ContentType)
		assert.Equal(t, "hello.txt", f.Name)
		assert.Equal(t, path, f.Path)
		assert.Equal(t, size, f.Size)
	})

	t.Run("returns uploaded files in the root folder", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		tkn, u, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		body := "hello world!"
		path := "/hello.txt"
		size := len(body)

		id := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path(path),
			Size:        file.Size(size),
		}, u.ID))

		_ = testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader("")),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/path/to/something/else.txt"),
			Size:        file.Size(0),
		}, u.ID))

		var resp api.FileList

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/files").
			Query("prefix", "/").
			WithContext(ctx).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&resp)

		assert.Equal(t, 1, resp.Count)
		assert.Equal(t, 1, resp.Total)
		require.Nil(t, resp.Next)
		assert.Equal(t, len(resp.Items), resp.Count)

		f := resp.Items[0]
		assert.Equal(t, id.AsUUID(), f.Id)
		assert.Equal(t, api.FileTypeFile, f.Type)
		assert.Equal(t, "text/plain", f.ContentType)
		assert.Equal(t, "hello.txt", f.Name)
		assert.Equal(t, path, f.Path)
		assert.Equal(t, size, f.Size)
	})
}

func TestApi_Files_Upload(t *testing.T) {
	t.Run("uploads a new file", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		dir := testutil.TempDirFromContext(ctx, t)

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		tkn, u, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		path := "/test.txt"
		content := []byte("hello world!")
		size := len(content)
		fpath := filepath.Join(dir, "test.txt")

		require.NoError(t, os.WriteFile(fpath, content, 0700))

		var resp api.FileUploadResponse

		apitest.New().
			Debug().
			Handler(handler).
			Post("/api/files").
			WithContext(ctx).
			MultipartFormData("path", path).
			MultipartFile("file", fpath).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&resp)

		assert.True(t, resp.Ok)
		assert.NotEmpty(t, resp.Id)

		fi, err := app.Files().ByID(ctx, testutil.Must(file.ParseID(resp.Id)))
		require.NoError(t, err)
		assert.NotNil(t, fi)
		assert.Equal(t, fi.Path, file.Path(path))
		assert.Equal(t, fi.OwnerID, u.ID)

		var list api.FileList

		apitest.New().
			Debug().
			Handler(handler).
			Get("/api/files").
			Query("prefix", "/").
			WithContext(ctx).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&list)

		assert.Equal(t, 1, list.Count)
		assert.Equal(t, 1, list.Total)
		require.Nil(t, list.Next)
		assert.Equal(t, len(list.Items), list.Count)

		f := list.Items[0]
		assert.Equal(t, api.FileTypeFile, f.Type)
		assert.Equal(t, "text/plain; charset=utf-8", f.ContentType)
		assert.Equal(t, "test.txt", f.Name)
		assert.Equal(t, path, f.Path)
		assert.Equal(t, size, f.Size)

	})
}
