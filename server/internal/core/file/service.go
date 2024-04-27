package file

import (
	"context"
	"fmt"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/rs/zerolog"
)

type Service struct {
	filesContent  FilesContent
	filesMetadata FilesMetadata
}

func NewService(filesContent FilesContent, filesMetadata FilesMetadata) Service {
	return Service{
		filesContent:  filesContent,
		filesMetadata: filesMetadata,
	}
}

func (s *Service) Upload(ctx context.Context, u user.User, content Content, path Path, size Size, logger zerolog.Logger) error {
	f, err := NewFile(content, path, size)
	if err != nil {
		return err
	}

	infraLogger := logger.With().Str("path", string(f.Path)).Int("size", int(f.Size)).Logger()

	if err := s.filesContent.Store(ctx, u, *f, infraLogger); err != nil {
		return fmt.Errorf("failed to store the file content: %w", err)
	}

	if _, err = s.filesMetadata.Save(ctx, u, *f, infraLogger); err != nil {
		return fmt.Errorf("failed to save the file metadata: %w", err)
	}

	return nil
}
