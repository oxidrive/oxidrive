package user

import (
	"context"

	"github.com/google/uuid"

	"github.com/oxidrive/oxidrive/server/internal/core/password"
)

type UserID (uuid.UUID)

func (i UserID) String() string {
	return uuid.UUID(i).String()
}

type User struct {
	ID           UserID
	Username     string
	PasswordHash password.Hash
}

func Create(username string, pwd string) (User, error) {
	hash, err := password.ValidateAndHash(pwd)
	if err != nil {
		return User{}, err
	}

	return User{
		ID:           UserID(uuid.Must(uuid.NewV7())),
		Username:     username,
		PasswordHash: hash,
	}, nil
}

func (u User) VerifyPassword(password string) (bool, error) {
	return u.PasswordHash.Verify(password)
}

type Users interface {
	Count(context.Context) (int, error)
	Save(context.Context, User) (User, error)
}
