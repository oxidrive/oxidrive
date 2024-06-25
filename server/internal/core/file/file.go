package file

import (
	"context"
	"errors"
	"fmt"
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
type ContentType string

func Close(c Content) error {
	cc, ok := c.(io.Closer)
	if !ok {
		return nil
	}

	return cc.Close()
}

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
	ErrInvalidPath  = errors.New("the provided file path is invalid")
	ErrFolderSave   = errors.New("cannot persist a folder")
	ErrFolderUpdate = errors.New("cannot update a folder")
)

type Type string

const (
	TypeFile   Type = "file"
	TypeFolder Type = "folder"

	ContentTypeFolder ContentType = "application/x-folder"
)

type Folder struct {
	Name Name
	Path Path
}

type File struct {
	ID          ID
	Type        Type
	ContentType ContentType
	Content     Content
	Name        Name
	Path        Path
	Size        Size
	OwnerID     user.ID
}

func Create(content Content, ct ContentType, p Path, size Size, ownerID user.ID) (*File, error) {
	p, err := ParsePath(string(p))
	if err != nil {
		return nil, err
	}

	name := Name(path.Base(string(p)))

	return &File{
		ID:          NewID(),
		Type:        TypeFile,
		Content:     content,
		ContentType: ct,
		Name:        name,
		Path:        p,
		Size:        size,
		OwnerID:     ownerID,
	}, nil
}

func (f *File) Update(content Content, ct ContentType, p Path, size Size) error {
	if f.Type != TypeFile {
		return ErrFolderUpdate
	}

	p, err := ParsePath(string(p))
	if err != nil {
		return err
	}

	f.Content = content
	f.ContentType = ct
	f.Name = p.Name()
	f.Path = p
	f.Size = size
	return nil
}

func (f *File) Folder() *Folder {
	p := path.Dir(string(f.Path))
	if p == "/" {
		return nil
	}

	n := path.Base(p)
	return &Folder{
		Name: Name(n),
		Path: Path(p),
	}
}

func ParsePath(p string) (Path, error) {
	if p == "" {
		return Path("/"), nil
	}

	cleaned := path.Clean(p)

	if strings.HasPrefix(cleaned, "../") {
		return Path(""), ErrInvalidPath
	}

	if !path.IsAbs(cleaned) {
		cleaned = fmt.Sprintf("/%s", cleaned)
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
	Load(context.Context, File) (Content, error)
	Delete(context.Context, File) error
}

type Files interface {
	List(ctx context.Context, prefix *Path, params list.Params) (list.Of[File], error)
	Save(context.Context, File) (*File, error)
	Delete(context.Context, File) error
	ByID(context.Context, ID) (*File, error)
	ByOwnerByPath(context.Context, user.ID, Path) (*File, error)
}
