package web

import (
	"context"
	"net/http"
	"os"
	"path/filepath"
	"time"

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

func TestApi_Files(t *testing.T) {
	t.Run("uploads a new file", func(t *testing.T) {
		ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
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

		path := "hello/test.txt"
		fpath := filepath.Join(dir, "test.txt")

		require.NoError(t, os.WriteFile(fpath, []byte("hello world!"), 0700))

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
	})
}
