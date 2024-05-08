package web

import (
	"context"
	"net/http"
	"os"
	"path/filepath"

	"testing"

	"github.com/go-http-utils/headers"
	"github.com/steinfletcher/apitest"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestApi_Files(t *testing.T) {
	t.Run("creates a new session for a user", func(t *testing.T) {
		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir(), testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		dir := testutil.TempDirFromContext(ctx, t)

		app, handler := setup(ctx, t)
		u := testutil.Must(app.Users().Save(ctx, *testutil.Must(user.Create("test", "test"))))
		tkn := testutil.Must(app.TokenVerifier().Generate(ctx, u))

		file := filepath.Join(dir, "test.txt")
		require.NoError(t, os.WriteFile(file, []byte("hello world!"), 0700))

		apitest.New().
			Debug().
			Handler(handler).
			Post("/api/files").
			MultipartFile("file", file).
			Header(headers.Authorization, "Bearer "+tkn.Value.String()).
			Expect(t).
			Status(http.StatusOK).
			Body(`{}`).
			End()
	})
}
