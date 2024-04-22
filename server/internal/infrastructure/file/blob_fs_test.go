package file

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestBlobFS_Store(t *testing.T) {
	contentStr := "This is a test!"
	content := strings.NewReader(contentStr)
	u, _ := user.Create("username", "pwd")

	t.Run("stores a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		blob := blobFS{filesRoot: path}
		f, _ := file.NewFile(content, "this/dir/without_error.txt", file.Size(len([]byte(contentStr))))

		err := blob.Store(context.Background(), *u, *f, zerolog.Nop())

		require.NoError(t, err)
		testFileContet(t, filepath.Join(path, u.ID.String(), string(f.Path)), contentStr)
	})

	t.Run("stores a file with timedouted context", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		blob := blobFS{filesRoot: path}
		f, _ := file.NewFile(content, "this/dir/timeout_error.txt", file.Size(len([]byte(contentStr))))
		ctx, cancel := context.WithTimeout(context.Background(), 0*time.Nanosecond)
		defer cancel()

		err := blob.Store(ctx, *u, *f, zerolog.Nop())

		require.Error(t, err)
		_, err = os.Stat(filepath.Join(path, u.ID.String(), string(f.Path)))
		require.ErrorIs(t, err, os.ErrNotExist)
	})

	t.Run("stores a file with cancelled context", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		blob := blobFS{filesRoot: path}
		f, _ := file.NewFile(content, "this/dir/ctx_cancelled_error.txt", file.Size(len([]byte(contentStr))))
		ctx, cancel := context.WithCancel(context.Background())
		cancel()

		err := blob.Store(ctx, *u, *f, zerolog.Nop())

		require.Error(t, err)
		_, err = os.Stat(filepath.Join(path, u.ID.String(), string(f.Path)))
		require.ErrorIs(t, err, os.ErrNotExist)
	})
}

func testFileContet(t *testing.T, filepath string, expected string) {
	t.Helper()

	f, err := os.Open(filepath)
	require.NoError(t, err)
	defer f.Close()

	toRead := make([]byte, len([]byte(expected)))
	read, err := f.Read(toRead)

	require.NoError(t, err)
	require.Equal(t, len([]byte(expected)), read)
	require.Equal(t, expected, string(toRead))
}
