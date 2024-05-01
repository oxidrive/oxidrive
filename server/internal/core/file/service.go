package file

import (
	"context"
	"fmt"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type FileUpload struct {
	Content Content
	Path    Path
	Size    Size
}

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

func (s *Service) Upload(ctx context.Context, toUpload FileUpload, owner user.ID) error {
	// TODO add user validation logic
	f, err := Create(toUpload.Content, toUpload.Path, toUpload.Size, owner)
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
