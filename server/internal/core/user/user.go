package user

import (
	"context"

	"github.com/google/uuid"

	"github.com/oxidrive/oxidrive/server/internal/core/password"
)

type ID (uuid.UUID)

func (i ID) String() string {
	return uuid.UUID(i).String()
}

type User struct {
	ID           ID
	Username     string
	PasswordHash password.Hash
}

func Create(username string, pwd string) (*User, error) {
	hash, err := password.ValidateAndHash(pwd)
	if err != nil {
		return nil, err
	}

	return &User{
		ID:           ID(uuid.Must(uuid.NewV7())),
		Username:     username,
		PasswordHash: hash,
	}, nil
}

func (u User) VerifyPassword(password string) (bool, error) {
	return u.PasswordHash.Verify(password)
}

type Users interface {
	Count(context.Context) (int, error)
	Save(context.Context, User) (*User, error)
}
