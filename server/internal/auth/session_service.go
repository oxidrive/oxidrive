package auth

import (
	"context"
	"errors"
	"time"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var ErrInvalidSession = errors.New("invalid session")

type SessionService struct {
	sessions    Sessions
	sessionsTTL time.Duration
}

func NewSessionService(tokens Sessions, tokenTTL time.Duration) SessionService {
	return SessionService{
		sessions:    tokens,
		sessionsTTL: tokenTTL,
	}
}

func (s *SessionService) Generate(ctx context.Context, u *user.User) (*Session, error) {
	return NewSession(u, time.Now().Add(s.sessionsTTL))
}

func (s *SessionService) Verify(ctx context.Context, token SessionID) error {
	session, err := s.sessions.ByID(ctx, token)
	if err != nil {
		return err
	}

	if session == nil || session.IsExpired() {
		return ErrInvalidSession
	}

	return nil
}

func (s *SessionService) ByID(ctx context.Context, id SessionID) (*Session, error) {
	return s.sessions.ByID(ctx, id)
}

func (s *SessionService) Store(ctx context.Context, token Session) (*Session, error) {
	return s.sessions.Store(ctx, token)
}
