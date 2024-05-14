package file

import (
	"context"
	"errors"
	"io"
	"path"
	"strings"

	"github.com/google/uuid"

	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type ID (uuid.UUID)
type Content io.Reader
type Path string
type Name string
type Size int

func EmptyID() ID {
	return ID(uuid.UUID{})
}

func NewID() ID {
	return ID(uuid.Must(uuid.NewV7()))
}

func ParseID(s string) (ID, error) {
	id, err := uuid.Parse(s)
	if err != nil {
		return ID{}, err
	}

	return ID(id), nil
}

func (i ID) AsUUID() uuid.UUID {
	return uuid.UUID(i)
}

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

func Create(content Content, p Path, size Size, ownerID user.ID) (*File, error) {
	p, err := ParsePath(string(p))
	if err != nil {
		return nil, err
	}

	name := Name(path.Base(string(p)))

	return &File{
		ID:      NewID(),
		Content: content,
		Name:    name,
		Path:    p,
		Size:    size,
		OwnerID: ownerID,
	}, nil
}

func (f *File) Update(content Content, p Path, size Size) error {
	p, err := ParsePath(string(p))
	if err != nil {
		return err
	}

	f.Content = content
	f.Name = p.Name()
	f.Path = p
	f.Size = size
	return nil
}

func ParsePath(p string) (Path, error) {
	cleaned := path.Clean(p)

	if path.IsAbs(cleaned) {
		cleaned = strings.Replace(cleaned, "/", "", 1)
	}

	if strings.HasPrefix(cleaned, "../") {
		return Path(""), ErrInvalidPath
	}

	return Path(cleaned), nil
}

func (p Path) Name() Name {
	return Name(path.Base(string(p)))
}

func (p Path) String() string {
	return string(p)
}

type Contents interface {
	Store(context.Context, File) error
}

type Files interface {
	List(ctx context.Context, prefix *Path, params list.Params) (list.Of[File], error)
	Save(context.Context, File) (*File, error)
	ByID(context.Context, ID) (*File, error)
	ByOwnerByPath(context.Context, user.ID, Path) (*File, error)
}
