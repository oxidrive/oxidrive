package file

import (
	"errors"
	"io"
	"path"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

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

type Name string
type Size int
type ContentType string

type Content io.Reader

func Close(c Content) error {
	cc, ok := c.(io.Closer)
	if !ok {
		return nil
	}

	return cc.Close()
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

func (f *File) UpdateContent(content Content, ct ContentType, size Size) error {
	if f.Type != TypeFile {
		return ErrFolderUpdate
	}

	f.Content = content
	f.ContentType = ct
	f.Size = size
	return nil
}

func (f *File) ChangePath(p Path) error {
	p, err := ParsePath(string(p))
	if err != nil {
		return err
	}

	f.Name = p.Name()
	f.Path = p
	return nil
}

type Folder struct {
	Name Name
	Path Path
}

func (f *File) Folder() *Folder {
	p := f.Path.Parent()
	if p.IsRoot() {
		return nil
	}

	return &Folder{
		Name: p.Name(),
		Path: p,
	}
}

func (f *File) Clone() File {
	return File{
		ID:          f.ID,
		Type:        f.Type,
		ContentType: f.ContentType,
		Content:     nil,
		Name:        f.Name,
		Path:        f.Path,
		Size:        f.Size,
		OwnerID:     f.OwnerID,
	}
}
