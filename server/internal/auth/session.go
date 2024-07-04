package auth

import (
	"time"

	"github.com/google/uuid"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type SessionID string

func (id SessionID) String() string {
	return string(id)
}

type Session struct {
	Value SessionID

	UserID    user.ID
	ExpiresAt time.Time
}

func (t *Session) IsExpired() bool {
	return t.ExpiresAt.Before(time.Now())
}

func NewSession(u *user.User, expiresAt time.Time) (*Session, error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return nil, err
	}

	return &Session{
		Value:     SessionID(id.String()),
		UserID:    u.ID,
		ExpiresAt: expiresAt,
	}, nil
}
