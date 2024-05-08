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

func NewService(filesContent Contents, filesMetadata Files) Service {
	return Service{
		contents: filesContent,
		files:    filesMetadata,
	}
}

func (s *Service) Upload(ctx context.Context, upload FileUpload, owner user.ID) error {
	f, err := s.files.ByOwnerByPath(ctx, owner, upload.Path)
	if err != nil {
		return err
	}

	if f == nil {
		f, err = Create(upload.Content, upload.Path, upload.Size, owner)
	} else {
		err = f.Update(upload.Content, upload.Path, upload.Size)
	}

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
