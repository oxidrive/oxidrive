package file

import (
	"context"
	"errors"
	"io"
	"path/filepath"

	"github.com/google/uuid"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/rs/zerolog"
)

type FileID (uuid.UUID)
type Content io.Reader
type Path string
type Name string
type Size int

func (i FileID) String() string {
	return uuid.UUID(i).String()
}

var (
	ErrInvalidPath = errors.New("the provided file path is invalid")
)

type File struct {
	ID      FileID
	Content Content
	Name    Name
	Path    Path
	Size    Size
}

func NewFile(content Content, path Path, size Size) (*File, error) {
	if !isValid(path) {
		return nil, ErrInvalidPath
	}

	name := Name(filepath.Base(string(path)))

	return &File{
		ID:      FileID(uuid.Must(uuid.NewV7())),
		Content: content,
		Name:    name,
		Path:    path,
		Size:    size,
	}, nil
}

func isValid(path Path) bool {
	cleaned := filepath.Clean(string(path))

	return filepath.IsLocal(cleaned)
}

type FilesContent interface {
	Store(context.Context, user.User, File, zerolog.Logger) error
}

type FilesMetadata interface {
	Save(context.Context, user.User, File, zerolog.Logger) (*File, error)
}
