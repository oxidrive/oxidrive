package file

import (
	"context"
	"fmt"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Service struct {
	filesContent  FilesContent
	filesMetadata FilesMetadata
}

func InitService(filesContent FilesContent, filesMetadata FilesMetadata) Service {
	return Service{
		filesContent:  filesContent,
		filesMetadata: filesMetadata,
	}
}

func (s *Service) Upload(ctx context.Context, content Content, path Path, size Size, ownerID user.ID) error {
	// TODO add user validation logic
	f, err := Create(content, path, size, ownerID)
	if err != nil {
		return err
	}

	if err := s.filesContent.Store(ctx, *f); err != nil {
		return fmt.Errorf("failed to store the file content: %w", err)
	}

	if _, err = s.filesMetadata.Save(ctx, *f); err != nil {
		return fmt.Errorf("failed to save the file metadata: %w", err)
	}

	return nil
}
