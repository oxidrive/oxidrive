package user

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"
)

type UsersMock struct {
	mock.Mock
}

func NewUsersMock(t *testing.T) *UsersMock {
	m := UsersMock{}
	m.Test(t)
	return &m
}

func (s *UsersMock) Count(ctx context.Context) (int, error) {
	args := s.Called(ctx)
	return args.Int(0), args.Error(1)
}

func (s *UsersMock) Save(ctx context.Context, user User) (*User, error) {
	args := s.Called(ctx, user)
	return args.Get(0).(*User), args.Error(1)
}
