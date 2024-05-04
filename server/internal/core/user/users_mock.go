package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"
)

var _ Users = (*UsersMock)(nil)

type UsersMock struct {
	mock.Mock
}

func NewUsersMock(t *testing.T) *UsersMock {
	m := UsersMock{}
	m.Test(t)
	return &m
}

func (s *UsersMock) Count(_ context.Context) (int, error) {
	args := s.Called()
	return args.Int(0), args.Error(1)
}

func (s *UsersMock) Save(_ context.Context, user User) (*User, error) {
	args := s.Called(user)
	return args.Get(0).(*User), args.Error(1)
}

func (s *UsersMock) ByID(_ context.Context, id ID) (*User, error) {
	args := s.Called(id)
	return args.Get(0).(*User), args.Error(1)
}

func (s *UsersMock) ByUsername(_ context.Context, username string) (*User, error) {
	args := s.Called(username)
	return args.Get(0).(*User), args.Error(1)
}
