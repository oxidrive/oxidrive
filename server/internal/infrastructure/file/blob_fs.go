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

const throughputInByte = 32

type blobFS struct {
	filesRoot string
}

func NewBlobFS(config config.StorageConfig) blobFS {
	return blobFS{
		filesRoot: config.StorageRoot,
	}
}

func (b *blobFS) Save(ctx context.Context, f file.File, logger zerolog.Logger) (err error) {
	fsPath := filepath.Join(b.filesRoot, string(f.Path))

	fsFile, err := os.OpenFile(fsPath, os.O_RDWR|os.O_CREATE, 0644)
	if err != nil {
		return err
	}
	defer func() {
		if clErr := fsFile.Close(); clErr != nil && !errors.Is(err, context.Canceled) && !errors.Is(err, context.DeadlineExceeded) {
			logger.Error().AnErr("error", clErr).Msg("error while closing the new file in blob fs save")
		}
	}()

	for {
		if err = ctx.Err(); err != nil {
			if reErr := os.Remove(string(f.Path)); reErr != nil {
				logger.Error().AnErr("error", reErr).Msg("error while removing the new file in blob fs save after context invalidation")
			}
			return fmt.Errorf("context invalidated while saving the new ifle in blob fs: %w", err)
		}

		if _, err = io.CopyN(fsFile, f.Content, throughputInByte); err != nil {
			if errors.Is(err, io.EOF) {
				return nil
			}
			return err
		}
	}
}
