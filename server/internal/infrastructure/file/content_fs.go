package file

import (
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/file"

	"github.com/rs/zerolog"
)

var _ file.Contents = (*contentFS)(nil)

const (
	filePermission      = 0644
	directoryPermission = 0755
)

type contentFS struct {
	dataDir     string
	filesPrefix string
	throughput  int
	logger      zerolog.Logger
}

func NewContentFS(cfg config.StorageConfig, logger zerolog.Logger) *contentFS {
	return &contentFS{
		dataDir:     cfg.StorageFSDataDir,
		filesPrefix: cfg.StoragePrefix,
		throughput:  cfg.ThroughputInByte,
		logger:      logger,
	}
}

func (c *contentFS) Store(ctx context.Context, f file.File) (err error) {
	fsPath := filepath.Join(c.dataDir, c.filesPrefix, f.OwnerID.String(), string(f.Path))
	if err := ensureDir(fsPath); err != nil {
		return err
	}

	logger := c.logger.With().Str("path", string(f.Path)).Int("size", int(f.Size)).Logger()
	fsFile, err := os.OpenFile(fsPath, os.O_RDWR|os.O_CREATE, filePermission)
	if err != nil {
		return err
	}
	defer func() {
		if clErr := fsFile.Close(); clErr != nil && !errors.Is(err, context.Canceled) && !errors.Is(err, context.DeadlineExceeded) {
			logger.Error().AnErr("error", clErr).Msg("error while closing the file in blob fs store")
		}
	}()

	for {
		if err = ctx.Err(); err != nil {
			if reErr := os.Remove(string(fsPath)); reErr != nil {
				logger.Error().AnErr("error", reErr).Msg("error while removing the file in blob fs store after context invalidation")
			}
			return fmt.Errorf("context invalidated while saving the file in blob fs: %w", err)
		}

		if _, err = io.CopyN(fsFile, f.Content, int64(c.throughput)); err != nil {
			if errors.Is(err, io.EOF) {
				return nil
			}
			return err
		}
	}
}

func ensureDir(path string) error {
	dir := filepath.Dir(path)
	return os.MkdirAll(dir, directoryPermission)
}
