package user

import (
	"github.com/google/uuid"
	"github.com/oxidrive/oxidrive/server/internal/core/password"
	"golang.org/x/exp/maps"
)

type UserId (uuid.UUID)

type User struct {
	Id           UserId
	Username     string
	PasswordHash password.Hash
}

func Create(username string, pwd string) (User, error) {
	hash, err := password.ValidateAndHash(pwd)
	if err != nil {
		return User{}, err
	}

	return User{
		Id:           UserId(uuid.Must(uuid.NewV7())),
		Username:     username,
		PasswordHash: hash,
	}, nil
}

func (u User) VerifyPassword(password string) (bool, error) {
	return u.PasswordHash.Verify(password)
}

type Users interface {
	Save(user User) (User, error)
}

// For testing only

type UsersSpy struct {
	users map[UserId]User
}

func NewUsersSpy() *UsersSpy {
	return &UsersSpy{
		users: make(map[UserId]User),
	}
}

func (s *UsersSpy) Users() []User {
	return maps.Values(s.users)
}

func (s *UsersSpy) Save(user User) (User, error) {
	s.users[user.Id] = user
	return user, nil
}
