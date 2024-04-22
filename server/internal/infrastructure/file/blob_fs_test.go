package file

import (
	"context"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/rs/zerolog"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestBlobFS_Save(t *testing.T) {
	blob := blobFS{}
	contentStr := "This is a test!"
	content := strings.NewReader(contentStr)

	t.Run("saves a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)

		filepath := path + "/without_error.txt"
		file := file.File{ID: file.FileID(uuid.Must(uuid.NewV7())), Path: file.Path(filepath), Name: file.Name("without_error.txt"), Content: content}

		err := blob.Save(context.Background(), file, zerolog.Nop())

		require.NoError(t, err)
		testFileContet(t, filepath, contentStr)
	})

	t.Run("saves a file with timedouted context", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)

		filepath := path + "/timeout_error.txt"
		file := file.File{ID: file.FileID(uuid.Must(uuid.NewV7())), Path: file.Path(filepath), Name: file.Name("timeout_error.txt"), Content: content}
		ctx, cancel := context.WithTimeout(context.Background(), 0*time.Nanosecond)
		defer cancel()

		err := blob.Save(ctx, file, zerolog.Nop())

		require.Error(t, err)
		_, err = os.Stat(filepath)
		require.ErrorIs(t, err, os.ErrNotExist)
	})

	t.Run("saves a file with cancelled context", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)

		filepath := path + "/ctx_cancelled_error.txt"
		file := file.File{ID: file.FileID(uuid.Must(uuid.NewV7())), Path: file.Path(filepath), Name: file.Name("ctx_cancelled_error.txt"), Content: content}
		ctx, cancel := context.WithCancel(context.Background())
		cancel()

		err := blob.Save(ctx, file, zerolog.Nop())

		require.Error(t, err)
		_, err = os.Stat(filepath)
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
