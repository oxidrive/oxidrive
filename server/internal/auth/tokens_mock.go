package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
)

var _ Tokens = (*TokensMock)(nil)

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

func (s *TokensMock) ExpiringBefore(_ context.Context, exp time.Time) ([]Token, error) {
	args := s.Called(exp)
	return args.Get(0).([]Token), args.Error(1)
}

func (s *TokensMock) DeleteAll(_ context.Context, tokens []Token) error {
	args := s.Called(tokens)
	return args.Error(0)
}
