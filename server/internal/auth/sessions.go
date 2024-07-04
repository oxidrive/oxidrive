package auth

import (
	"context"
	"time"
)

type Sessions interface {
	ExpiringBefore(context.Context, time.Time) ([]Session, error)
	ByID(context.Context, SessionID) (*Session, error)
	Store(context.Context, Session) (*Session, error)
	DeleteAll(context.Context, []Session) error
}
