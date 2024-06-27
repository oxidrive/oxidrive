package web

import (
	"context"
	"fmt"
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
	h "github.com/oxidrive/oxidrive/server/internal/web/handler"
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
			Cookie(h.SessionCookieName, tkn.Value.String()).
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
			Cookie(h.SessionCookieName, tkn.Value.String()).
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
			Cookie(h.SessionCookieName, tkn.Value.String()).
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
			Cookie(h.SessionCookieName, tkn.Value.String()).
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
			Cookie(h.SessionCookieName, tkn.Value.String()).
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

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob%s", f.Path).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			Header(headers.ContentLength, fmt.Sprintf("%d", size)).
			Header(headers.ContentDisposition, "attachment; filename="+f.Name).
			Body(string(content)).
			End()
	})
}

func TestApi_Files_Blob(t *testing.T) {
	t.Run("downloads a file blob", func(t *testing.T) {
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
		name := "hello.txt"
		size := len(body)

		_ = testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/" + name),
			Size:        file.Size(size),
		}, u.ID))

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob/%s", name).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			Header(headers.ContentLength, fmt.Sprintf("%d", size)).
			Header(headers.ContentDisposition, "attachment; filename="+name).
			Body(body).
			End()
	})
}

func TestApi_Files_Patch(t *testing.T) {
	t.Run("moves a file to a new path", func(t *testing.T) {
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
		oldPath := "/hello.txt"
		newName := "changed.txt"
		newPath := "/" + newName

		id := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path(oldPath),
			Size:        file.Size(size),
		}, u.ID))

		var f api.File

		apitest.New().
			Debug().
			Handler(handler).
			Patchf("/api/files/%s", id.String()).
			JSON(api.FilePatch{
				Path: &newPath,
			}).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&f)

		assert.Equal(t, id.AsUUID(), f.Id)
		assert.Equal(t, api.FileTypeFile, f.Type)
		assert.Equal(t, "text/plain", f.ContentType)
		assert.Equal(t, newName, f.Name)
		assert.Equal(t, newPath, f.Path)
		assert.Equal(t, size, f.Size)

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob%s", newPath).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			Header(headers.ContentLength, fmt.Sprintf("%d", size)).
			Header(headers.ContentDisposition, "attachment; filename="+newName).
			Body(body).
			End()

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob%s", oldPath).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusNotFound).
			End()
	})

	t.Run("moves a directory to a new path", func(t *testing.T) {
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
		oldPath := file.Path("/old-folder")
		newName := "new-folder"
		newPath := "/" + newName
		fileName := "file.txt"

		_ = testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path(string(oldPath) + "/" + fileName),
			Size:        file.Size(size),
		}, u.ID))

		files := testutil.Must(app.Files().List(ctx, nil))

		var dir file.File
		var f file.File

		for _, i := range files.Items {
			if i.Type == file.TypeFolder && i.Path == oldPath {
				dir = i
			}

			if i.Type == file.TypeFile && i.Name == file.Name(fileName) {
				f = i
			}
		}
		require.NotEmpty(t, dir)
		require.NotEmpty(t, f)

		var d api.File

		apitest.New().
			Debug().
			Handler(handler).
			Patchf("/api/files/%s", dir.ID.String()).
			JSON(api.FilePatch{
				Path: &newPath,
			}).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&d)

		assert.Equal(t, dir.ID.AsUUID(), d.Id)
		assert.Equal(t, api.FileTypeFolder, d.Type)
		assert.Equal(t, string(file.ContentTypeFolder), d.ContentType)
		assert.Equal(t, newName, d.Name)
		assert.Equal(t, newPath, d.Path)

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob%s", newPath+"/"+fileName).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			Header(headers.ContentLength, fmt.Sprintf("%d", size)).
			Header(headers.ContentDisposition, "attachment; filename="+fileName).
			Body(body).
			End()

		apitest.New().
			Debug().
			Handler(handler).
			Getf("/blob%s", string(oldPath)+"/"+fileName).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusNotFound).
			End()
	})

	t.Run("returns 404 if the file does not exist", func(t *testing.T) {})
	t.Run("returns 404 if the file belongs to another user", func(t *testing.T) {})
}

func TestApi_Files_Delete(t *testing.T) {
	t.Run("deletes an existing file", func(t *testing.T) {
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

		id := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/hello.txt"),
			Size:        file.Size(size),
		}, u.ID))

		var f api.File

		apitest.New().
			Debug().
			Handler(handler).
			Deletef("/api/files/%s", id.String()).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			End().
			JSON(&f)

		assert.Equal(t, id.AsUUID(), f.Id)
		assert.Equal(t, api.FileTypeFile, f.Type)
		assert.Equal(t, "text/plain", f.ContentType)
		assert.Equal(t, "hello.txt", f.Name)
		assert.Equal(t, "/hello.txt", f.Path)
		assert.Equal(t, size, f.Size)
	})

	t.Run("does not delete an existing file that belongs to another user", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		u := testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create("different", password))))
		tkn, _, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		body := "hello world!"
		size := len(body)

		id := testutil.Must(app.Files().Upload(ctx, file.FileUpload{
			Content:     file.Content(strings.NewReader(body)),
			ContentType: file.ContentType("text/plain"),
			Path:        file.Path("/hello.txt"),
			Size:        file.Size(size),
		}, u.ID))

		var e api.Error

		apitest.New().
			Debug().
			Handler(handler).
			Deletef("/api/files/%s", id.String()).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusNotFound).
			End().
			JSON(&e)

		assert.Equal(t, string(api.NotFoundErrorErrorNotFound), e.Error)
	})

	t.Run("does not delete a file that does not exist", func(t *testing.T) {
		t.Parallel()

		ctx, cancel := context.WithTimeout(context.Background(), timeout)
		defer cancel()

		ctx, done := testutil.IntegrationTest(ctx, t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		app, handler := setup(ctx, t)

		username := "test"
		password := "test"

		testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create(username, password))))
		tkn, _, err := app.Auth().AuthenticateWithPassword(ctx, username, password)
		require.NoError(t, err)

		id := file.NewID()

		var e api.Error

		apitest.New().
			Debug().
			Handler(handler).
			Deletef("/api/files/%s", id.String()).
			WithContext(ctx).
			Cookie(h.SessionCookieName, tkn.Value.String()).
			Expect(t).
			Status(http.StatusNotFound).
			End().
			JSON(&e)

		assert.Equal(t, string(api.NotFoundErrorErrorNotFound), e.Error)
	})
}
