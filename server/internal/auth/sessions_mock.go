package auth

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/mock"
)

var _ Sessions = (*SessionsMock)(nil)

type SessionsMock struct {
	mock.Mock
}

func NewSessionsMock(t *testing.T) *SessionsMock {
	m := SessionsMock{}
	m.Test(t)
	return &m
}

func (s *SessionsMock) ByID(_ context.Context, id SessionID) (*Session, error) {
	args := s.Called(id)
	return args.Get(0).(*Session), args.Error(1)
}

func (s *SessionsMock) Store(_ context.Context, session Session) (*Session, error) {
	args := s.Called(session)
	return args.Get(0).(*Session), args.Error(1)
}

func (s *SessionsMock) ExpiringBefore(_ context.Context, exp time.Time) ([]Session, error) {
	args := s.Called(exp)
	return args.Get(0).([]Session), args.Error(1)
}

func (s *SessionsMock) DeleteAll(_ context.Context, sessions []Session) error {
	args := s.Called(sessions)
	return args.Error(0)
}
