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

func (c *contentFS) Store(ctx context.Context, f file.File) error {
	fsPath := c.pathFor(f)
	if err := ensureDir(fsPath); err != nil {
		return err
	}

	s, ok := f.Content.(io.Seeker)
	if ok {
		if _, err := s.Seek(0, io.SeekStart); err != nil {
			return fmt.Errorf("failed to rewind file %s to start of buffer: %w", fsPath, err)
		}

	}

	logger := c.logger.With().
		Str("path", string(f.Path)).
		Str("owner_id", f.OwnerID.String()).
		Str("id", f.ID.String()).
		Int("size", int(f.Size)).
		Logger()
	fsFile, err := os.OpenFile(fsPath, os.O_RDWR|os.O_CREATE, filePermission)
	if err != nil {
		return err
	}
	defer func() {
		if clErr := fsFile.Close(); clErr != nil && !errors.Is(err, context.Canceled) && !errors.Is(err, context.DeadlineExceeded) {
			logger.Error().AnErr("error", clErr).Msg("error while closing the file in blob fs store")
		}
	}()

	var total int64
	for {
		if err = ctx.Err(); err != nil {
			if reErr := os.Remove(string(fsPath)); reErr != nil {
				logger.Error().AnErr("error", reErr).Msg("error while removing the file in blob fs store after context invalidation")
			}
			return fmt.Errorf("context invalidated while saving the file in blob fs: %w", err)
		}

		n, err := io.CopyN(fsFile, f.Content, int64(c.throughput))
		if err != nil {
			if errors.Is(err, io.EOF) {
				logger.Debug().Int64("bytes_written", total+n).Msg("stored file in filesystem")
				return nil
			}
			return err
		}
		total += n
	}
}

func (c *contentFS) Load(ctx context.Context, f file.File) (file.Content, error) {
	fsPath := c.pathFor(f)

	fsFile, err := os.Open(fsPath)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return nil, file.ErrFileNotFound
		}

		return nil, fmt.Errorf("failed to open file %s: %w", fsPath, err)
	}

	c.logger.Debug().
		Str("path", string(f.Path)).
		Str("owner_id", f.OwnerID.String()).
		Str("id", f.ID.String()).
		Msg("loaded file from filesystem")

	return fsFile, nil
}

func (c *contentFS) Delete(ctx context.Context, f file.File) error {
	fsPath := c.pathFor(f)

	if err := os.Remove(fsPath); err != nil {
		if errors.Is(err, os.ErrNotExist) {
			return file.ErrFileNotFound
		}

		return fmt.Errorf("failed to delete file %s: %w", fsPath, err)
	}

	c.logger.Debug().
		Str("path", string(f.Path)).
		Str("owner_id", f.OwnerID.String()).
		Str("id", f.ID.String()).
		Msg("deleted file from filesystem")

	return nil
}

func (c *contentFS) Copy(ctx context.Context, from file.File, to file.File) error {
	fromPath := c.pathFor(from)
	toPath := c.pathFor(to)

	if fromPath == toPath {
		// noop
		return nil
	}

	content, err := c.Load(ctx, from)
	if err != nil {
		return fmt.Errorf("failed to open source file for copy: %w", err)
	}

	to.Content = content
	return c.Store(ctx, to)
}

func (c *contentFS) pathFor(f file.File) string {
	return filepath.Join(c.dataDir, c.filesPrefix, f.OwnerID.String(), string(f.Path))
}

func ensureDir(path string) error {
	dir := filepath.Dir(path)
	return os.MkdirAll(dir, directoryPermission)
}
