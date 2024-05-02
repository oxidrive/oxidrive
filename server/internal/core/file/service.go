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
	contents Contents
	files    Files
}

func InitService(filesContent Contents, filesMetadata Files) Service {
	return Service{
		contents: filesContent,
		files:    filesMetadata,
	}
}

func (s *Service) Upload(ctx context.Context, upload FileUpload, owner user.ID) error {
	// TODO add user validation logic
	f, err := Create(upload.Content, upload.Path, upload.Size, owner)
	if err != nil {
		return err
	}

	if err := s.contents.Store(ctx, *f); err != nil {
		return fmt.Errorf("failed to store the file content: %w", err)
	}

	if _, err = s.files.Save(ctx, *f); err != nil {
		return fmt.Errorf("failed to save the file metadata: %w", err)
	}

	return nil
}
