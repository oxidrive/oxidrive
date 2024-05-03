package file

import (
	"context"
	"errors"
	"io"
	"path/filepath"

	"github.com/google/uuid"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type ID (uuid.UUID)
type Content io.Reader
type Path string
type Name string
type Size int

func (i ID) String() string {
	return uuid.UUID(i).String()
}

var (
	ErrInvalidPath = errors.New("the provided file path is invalid")
)

type File struct {
	ID      ID
	Content Content
	Name    Name
	Path    Path
	Size    Size
	OwnerID user.ID
}

func Create(content Content, path Path, size Size, ownerID user.ID) (*File, error) {
	if !isValid(path) {
		return nil, ErrInvalidPath
	}

	name := Name(filepath.Base(string(path)))

	return &File{
		ID:      ID(uuid.Must(uuid.NewV7())),
		Content: content,
		Name:    name,
		Path:    Path(filepath.Clean(string(path))),
		Size:    size,
		OwnerID: ownerID,
	}, nil
}

func (f *File) Update(content Content, path Path, size Size) error {
	if !isValid(path) {
		return ErrInvalidPath
	}

	f.Content = content
	f.Name = Name(filepath.Base(string(path)))
	f.Path = Path(filepath.Clean(string(path)))
	f.Size = size
	return nil
}

func isValid(path Path) bool {
	cleaned := filepath.Clean(string(path))

	return filepath.IsLocal(cleaned)
}

type Contents interface {
	Store(context.Context, File) error
}

type Files interface {
	Save(context.Context, File) (*File, error)
	ByOwnerByPath(context.Context, user.ID, Path) (*File, error)
}
