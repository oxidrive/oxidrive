package file

import (
	"context"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

var (
	contentStr  string              = "This is a test!"
	fileContent func() file.Content = func() file.Content { return strings.NewReader(contentStr) }
	size        file.Size           = file.Size(len([]byte(contentStr)))
)

func TestContentFS_Store(t *testing.T) {
	l := zerolog.New(zerolog.NewTestWriter(t))

	t.Run("stores a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/without_error.txt", size, user.NewID())
		require.NoError(t, err)

		_, err = io.Copy(io.Discard, f.Content)
		require.NoError(t, err)

		err = content.Store(context.Background(), *f)
		require.NoError(t, err)

		testFileContent(t, filepath.Join(path, f.OwnerID.String(), string(f.Path)), contentStr)
	})

	t.Run("stores a file rewinding the reader if necessary", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/without_error.txt", size, user.NewID())
		require.NoError(t, err)

		err = content.Store(context.Background(), *f)
		require.NoError(t, err)

		testFileContent(t, filepath.Join(path, f.OwnerID.String(), string(f.Path)), contentStr)
	})

	t.Run("doesn't store a file if the context times out", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/timeout_error.txt", size, user.NewID())
		require.NoError(t, err)
		ctx, cancel := context.WithTimeout(context.Background(), 0*time.Nanosecond)
		defer cancel()

		err = content.Store(ctx, *f)

		require.Error(t, err)
		_, err = os.Stat(filepath.Join(path, f.OwnerID.String(), string(f.Path)))
		require.ErrorIs(t, err, os.ErrNotExist)
	})

	t.Run("doesn't store a file if the context is cancelled", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/ctx_cancelled_error.txt", size, user.NewID())
		require.NoError(t, err)
		ctx, cancel := context.WithCancel(context.Background())
		cancel()

		err = content.Store(ctx, *f)

		require.Error(t, err)
		_, err = os.Stat(filepath.Join(path, f.OwnerID.String(), string(f.Path)))
		require.ErrorIs(t, err, os.ErrNotExist)
	})
}

func TestContentFS_Load(t *testing.T) {
	l := zerolog.New(zerolog.NewTestWriter(t))

	t.Run("deletes a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/file.txt", size, user.NewID())
		require.NoError(t, err)

		err = content.Store(ctx, *f)
		require.NoError(t, err)

		c, err := content.Load(ctx, *f)
		require.NoError(t, err)
		require.NotNil(t, c)
		defer file.Close(c)

		b, err := io.ReadAll(c)
		require.NoError(t, err)
		require.Equal(t, []byte(contentStr), b)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/missing.txt", size, user.NewID())
		require.NoError(t, err)

		c, err := content.Load(ctx, *f)
		require.Equal(t, file.ErrFileNotFound, err)
		require.Nil(t, c)
	})

}

func TestContentFS_Copy(t *testing.T) {
	l := zerolog.New(zerolog.NewTestWriter(t))

	t.Run("copies a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)

		from, err := file.Create(fileContent(), "text/plain", "/original.txt", size, user.NewID())
		require.NoError(t, err)

		to := from.Clone()
		to.Path = "copied.txt"

		err = content.Store(ctx, *from)
		require.NoError(t, err)

		testFileContent(t, filepath.Join(path, from.OwnerID.String(), string(from.Path)), contentStr)

		err = content.Copy(ctx, *from, to)
		require.NoError(t, err)

		original, err := content.Load(ctx, *from)
		require.NoError(t, err)
		require.NotNil(t, original)
		defer file.Close(original)

		copied, err := content.Load(ctx, to)
		require.NoError(t, err)
		require.NotNil(t, copied)
		defer file.Close(copied)

		o, err := io.ReadAll(original)
		require.NoError(t, err)
		require.Equal(t, []byte(contentStr), o)

		c, err := io.ReadAll(copied)
		require.NoError(t, err)
		require.Equal(t, []byte(contentStr), c)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)

		from, err := file.Create(fileContent(), "text/plain", "this/dir/missing.txt", size, user.NewID())
		require.NoError(t, err)

		to := from.Clone()
		to.Path = "copied.txt"

		err = content.Copy(ctx, *from, to)
		require.ErrorIs(t, err, file.ErrFileNotFound)

		c, err := content.Load(ctx, to)
		require.Equal(t, file.ErrFileNotFound, err)
		require.Nil(t, c)
	})
}

func TestContentFS_Delete(t *testing.T) {
	l := zerolog.New(zerolog.NewTestWriter(t))

	t.Run("deletes a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/deleted.txt", size, user.NewID())
		require.NoError(t, err)

		err = content.Store(ctx, *f)
		require.NoError(t, err)

		testFileContent(t, filepath.Join(path, f.OwnerID.String(), string(f.Path)), contentStr)

		err = content.Delete(ctx, *f)
		require.NoError(t, err)

		c, err := content.Load(ctx, *f)
		require.Equal(t, file.ErrFileNotFound, err)
		require.Nil(t, c)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithTempDir())
		defer done()

		path := testutil.TempDirFromContext(ctx, t)
		content := NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
		f, err := file.Create(fileContent(), "text/plain", "this/dir/missing.txt", size, user.NewID())
		require.NoError(t, err)

		err = content.Delete(ctx, *f)
		require.ErrorIs(t, err, file.ErrFileNotFound)
	})
}

func testFileContent(t *testing.T, filepath string, expected string) {
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
