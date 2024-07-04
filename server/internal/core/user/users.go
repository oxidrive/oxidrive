package user

import "context"

type Users interface {
	Count(context.Context) (int, error)
	Save(context.Context, User) (*User, error)
	ByID(context.Context, ID) (*User, error)
	ByUsername(context.Context, string) (*User, error)
}

