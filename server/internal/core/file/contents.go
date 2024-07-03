package file

import "context"

type Contents interface {
	Store(context.Context, File) error
	Load(context.Context, File) (Content, error)
	Delete(context.Context, File) error
	Copy(ctx context.Context, from File, to File) error
}
