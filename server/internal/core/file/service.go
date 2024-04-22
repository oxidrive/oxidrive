package file

import (
	"context"
	"fmt"

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

func (s *Service) Upload(ctx context.Context, content Content, path Path, size Size, logger zerolog.Logger) error {
	file, err := NewFile(content, path, size)
	if err != nil {
		return err
	}

	infraLogger := logger.With().Str("path", string(file.Path)).Int("size", int(file.Size)).Logger()

	if err := s.filesContent.Store(ctx, *file, infraLogger); err != nil {
		return fmt.Errorf("failed to store the file content: %w", err)
	}

	if _, err = s.filesMetadata.Save(ctx, *file, infraLogger); err != nil {
		return fmt.Errorf("failed to save the file metadata: %w", err)
	}

	return nil
}
