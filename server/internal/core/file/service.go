package file

import (
	"context"
	"errors"
	"fmt"
	"strings"

	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var (
	ErrFileNotFound error = errors.New("file does not exist")
)

type FileUpload struct {
	Content     Content
	ContentType ContentType
	Path        Path
	Size        Size
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

func (s *Service) List(ctx context.Context, prefix *Path, params ...list.Param) (list.Of[File], error) {
	return s.files.List(ctx, prefix, list.NewParams(params...))
}

func (s *Service) ByID(ctx context.Context, id ID) (*File, error) {
	return s.files.ByID(ctx, id)
}

func (s *Service) ByOwnerByPath(ctx context.Context, owner user.ID, path Path) (*File, error) {
	return s.files.ByOwnerByPath(ctx, owner, path)
}

func (s *Service) Upload(ctx context.Context, upload FileUpload, owner user.ID) (ID, error) {
	f, err := s.files.ByOwnerByPath(ctx, owner, upload.Path)
	if err != nil {
		return EmptyID(), err
	}

	if f == nil {
		f, err = Create(upload.Content, upload.ContentType, upload.Path, upload.Size, owner)
	} else {
		err = f.UpdateContent(upload.Content, upload.ContentType, upload.Size)
	}

	if err != nil {
		return EmptyID(), err
	}

	if err := s.contents.Store(ctx, *f); err != nil {
		return EmptyID(), fmt.Errorf("failed to store the file content: %w", err)
	}

	if f, err = s.files.Save(ctx, *f); err != nil {
		return EmptyID(), fmt.Errorf("failed to save the file metadata: %w", err)
	}

	return f.ID, nil
}

func (s *Service) Download(ctx context.Context, f File) (Content, error) {
	return s.contents.Load(ctx, f)
}

func (s *Service) Move(ctx context.Context, f File, newPath Path) (*File, error) {
	old := f.Clone()

	if err := f.ChangePath(newPath); err != nil {
		return nil, fmt.Errorf("failed to change path of %s %s: %w", f.Type, f.ID, err)
	}

	if f.Type == TypeFolder {
		return s.moveFolder(ctx, old, f)
	}

	return s.moveFile(ctx, old, f)
}

func (s *Service) moveFile(ctx context.Context, from File, to File) (*File, error) {
	if err := s.contents.Copy(ctx, from, to); err != nil {
		return nil, fmt.Errorf("failed to copy file %s content from %s to %s: %w", to.ID, from.Path, to.Path, err)
	}

	updated, err := s.files.Save(ctx, to)
	if err != nil {
		return nil, fmt.Errorf("failed to store file %s: %w", to.ID, err)
	}

	if err := s.contents.Delete(ctx, from); err != nil {
		return nil, fmt.Errorf("failed to delete file %s content from %s: %w", to.ID, from.Path, err)
	}

	return updated, nil
}

type childError struct {
	parent File
	child  File
	err    error
}

func (ce childError) Error() string {
	return fmt.Errorf("failed to move child %s of %s: %w", ce.child.ID, ce.parent.ID, ce.err).Error()
}

func (s *Service) moveFolder(ctx context.Context, from File, to File) (*File, error) {
	updated, err := s.files.Save(ctx, to)
	if err != nil {
		return nil, fmt.Errorf("failed to store folder %s: %w", to.ID, err)
	}

	children, err := s.List(ctx, &from.Path)
	if err != nil {
		return nil, fmt.Errorf("failed to list children of folder %s: %w", from.ID, err)
	}

	if children.Count > 0 {
		var errs []error

		for _, child := range children.Items {
			newPath := Path(strings.Replace(string(child.Path), string(from.Path), string(to.Path), 1))
			_, err := s.Move(ctx, child, newPath)
			if err != nil {
				errs = append(errs, childError{from, child, err})
			}
		}

		if len(errs) > 0 {
			return nil, errors.Join(errs...)
		}
	}

	return updated, err
}

func (s *Service) Delete(ctx context.Context, id ID) error {
	f, err := s.files.ByID(ctx, id)
	if err != nil {
		return fmt.Errorf("failed to load file %s: %w", id, err)
	}

	if f == nil {
		return ErrFileNotFound
	}

	if err := s.files.Delete(ctx, *f); err != nil {
		// if the metadata are missing then we don't really care. Our goal was to remove it after all
		if errors.Is(err, ErrFileNotFound) {
			return fmt.Errorf("failed to delete metadata for file %s: %w", id, err)
		}
	}

	if err := s.contents.Delete(ctx, *f); err != nil {
		// if the content is missing then we don't really care. Our goal was to remove it after all
		if !errors.Is(err, ErrFileNotFound) {
			return fmt.Errorf("failed to delete content of file %s: %w", id, err)
		}
	}

	return nil
}
