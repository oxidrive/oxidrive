package file

import (
	"context"
	"testing"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func fsInit(t *testing.T, ctx context.Context) file.Contents {
	l := zerolog.New(zerolog.NewTestWriter(t))
	path := testutil.TempDirFromContext(ctx, t)
	return NewContentFS(config.StorageConfig{StoragePrefix: path, ThroughputInByte: 32}, l)
}

func TestContentFS_Store(t *testing.T) {
	ContentsStore(t, testutil.WithTempDir(), fsInit)
}

func TestContentFS_Load(t *testing.T) {
	ContentsLoad(t, testutil.WithTempDir(), fsInit)
}

func TestContentFS_Copy(t *testing.T) {
	ContentsCopy(t, testutil.WithTempDir(), fsInit)
}

func TestContentFS_Delete(t *testing.T) {
	ContentsDelete(t, testutil.WithTempDir(), fsInit)
}
