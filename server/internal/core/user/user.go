package user

import (
	"context"

	"github.com/google/uuid"
	"github.com/oxidrive/oxidrive/server/internal/core/password"
	"golang.org/x/exp/maps"
)

type UserId (uuid.UUID)

func (i UserId) String() string {
	return uuid.UUID(i).String()
}

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
	Count(context.Context) (int, error)
	Save(context.Context, User) (User, error)
}

// For testing only

type UsersSpy struct {
	users map[UserId]User
}

func NewUsersSpy(existing ...User) *UsersSpy {
	users := make(map[UserId]User)
	for _, u := range existing {
		users[u.Id] = u
	}

	return &UsersSpy{
		users: users,
	}
}

func (s *UsersSpy) Users() []User {
	return maps.Values(s.users)
}

// impl users.Users

func (s *UsersSpy) Count(_ context.Context) (int, error) {
	return len(s.users), nil
}

func (s *UsersSpy) Save(_ context.Context, user User) (User, error) {
	s.users[user.Id] = user
	return user, nil
}
