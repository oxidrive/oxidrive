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

const (
	throughputInByte    = 32
	filePermission      = 0644
	directoryPermission = 0755
)

type blobFS struct {
	filesPrefix string
	logger      zerolog.Logger
}

func NewBlobFS(config config.StorageConfig, logger zerolog.Logger) *blobFS {
	return &blobFS{
		filesPrefix: config.StoragePrefix,
		logger:      logger,
	}
}

func (b *blobFS) Store(ctx context.Context, f file.File) (err error) {
	fsPath := filepath.Join(b.filesPrefix, f.OwnerID.String(), string(f.Path))
	if err := ensureDir(fsPath); err != nil {
		return err
	}

	logger := b.logger.With().Str("path", string(f.Path)).Int("size", int(f.Size)).Logger()
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

		if _, err = io.CopyN(fsFile, f.Content, throughputInByte); err != nil {
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
