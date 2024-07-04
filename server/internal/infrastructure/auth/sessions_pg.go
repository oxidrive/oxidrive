package auth

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/google/uuid"
	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ auth.Sessions = (*PgSessions)(nil)

type PgSessions struct {
	db *sqlx.DB
}

func NewPgSessions(db *sqlx.DB) *PgSessions {
	return &PgSessions{db: db}
}

func (p *PgSessions) ByID(ctx context.Context, id auth.SessionID) (*auth.Session, error) {
	var t pgSession
	err := p.db.GetContext(ctx, &t, "select id, user_id, expires_at from sessions where id = $1", string(id))
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return t.into(), nil
}

func (p *PgSessions) Store(ctx context.Context, session auth.Session) (*auth.Session, error) {
	_, err := p.db.ExecContext(ctx, `insert into sessions (
        id,
        user_id,
        expires_at
    ) values (
        $1,
        $2,
        $3
    )`, session.Value.String(), session.UserID.String(), session.ExpiresAt)
	if err != nil {
		return nil, err
	}

	return &session, nil
}

func (p *PgSessions) ExpiringBefore(ctx context.Context, exp time.Time) ([]auth.Session, error) {
	ptt := make([]pgSession, 0)
	err := p.db.SelectContext(ctx, &ptt, "select id, user_id, expires_at from sessions where expires_at <= $1", exp)
	if errors.Is(err, sql.ErrNoRows) {
		return []auth.Session{}, nil
	}

	if err != nil {
		return nil, err
	}

	tt := make([]auth.Session, len(ptt))

	for i, t := range ptt {
		tt[i] = *t.into()
	}

	return tt, nil
}

func (p *PgSessions) DeleteAll(ctx context.Context, sessions []auth.Session) error {
	if len(sessions) == 0 {
		return nil
	}

	ids := make([]string, len(sessions))
	for i, t := range sessions {
		ids[i] = t.Value.String()
	}

	query, args, err := sqlx.In("delete from sessions where id in (?)", ids)
	if err != nil {
		return err
	}

	query = p.db.Rebind(query)

	_, err = p.db.ExecContext(ctx, query, args...)
	return err
}

type pgSession struct {
	ID        string    `db:"id"`
	UserID    uuid.UUID `db:"user_id"`
	ExpiresAt time.Time `db:"expires_at"`
}

func (t pgSession) into() *auth.Session {
	return &auth.Session{
		Value:     auth.SessionID(t.ID),
		UserID:    user.ID(t.UserID),
		ExpiresAt: t.ExpiresAt,
	}
}
