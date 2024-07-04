package user

import (
	"github.com/oxidrive/oxidrive/server/internal/core/password"
)

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
		ID:           NewID(),
		Username:     username,
		PasswordHash: hash,
	}, nil
}

func (u User) VerifyPassword(password string) (bool, error) {
	return u.PasswordHash.Verify(password)
}
