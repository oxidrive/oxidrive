package file

import (
	"context"

	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type Files interface {
	List(ctx context.Context, prefix *Path, params list.Params) (list.Of[File], error)
	Save(context.Context, File) (*File, error)
	Delete(context.Context, File) error
	ByID(context.Context, ID) (*File, error)
	ByOwnerByPath(context.Context, user.ID, Path) (*File, error)
}
