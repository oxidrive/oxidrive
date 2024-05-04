package auth

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"
)

type TokensMock struct {
	mock.Mock
}

func NewTokensMock(t *testing.T) *TokensMock {
	m := TokensMock{}
	m.Test(t)
	return &m
}

func (s *TokensMock) ByID(_ context.Context, id TokenID) (*Token, error) {
	args := s.Called(id)
	return args.Get(0).(*Token), args.Error(1)
}

func (s *TokensMock) Store(_ context.Context, token Token) (*Token, error) {
	args := s.Called(token)
	return args.Get(0).(*Token), args.Error(1)
}
