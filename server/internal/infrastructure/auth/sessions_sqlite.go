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

var _ auth.Sessions = (*SqliteSessions)(nil)

type SqliteSessions struct {
	db *sqlx.DB
}

func NewSqliteSessions(db *sqlx.DB) *SqliteSessions {
	return &SqliteSessions{db: db}
}

func (p *SqliteSessions) ByID(ctx context.Context, id auth.SessionID) (*auth.Session, error) {
	var t sqliteSession
	err := p.db.GetContext(ctx, &t, "select id, user_id, expires_at from sessions where id = $1", string(id))
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	return t.into(), nil
}

func (p *SqliteSessions) Store(ctx context.Context, t auth.Session) (*auth.Session, error) {
	exp := t.ExpiresAt.UTC().Format(time.RFC3339)
	_, err := p.db.ExecContext(ctx, `insert into sessions (
        id,
        user_id,
        expires_at
    ) values (
        $1,
        $2,
        $3
    )`, t.Value.String(), t.UserID.String(), exp)
	if err != nil {
		return nil, err
	}

	return &t, nil
}

func (p *SqliteSessions) ExpiringBefore(ctx context.Context, exp time.Time) ([]auth.Session, error) {
	e := exp.UTC().Format(time.RFC3339)
	stt := make([]sqliteSession, 0)
	err := p.db.SelectContext(ctx, &stt, "select id, user_id, expires_at from sessions where expires_at <= $1", e)
	if errors.Is(err, sql.ErrNoRows) {
		return []auth.Session{}, nil
	}

	if err != nil {
		return nil, err
	}

	tt := make([]auth.Session, len(stt))

	for i, t := range stt {
		tt[i] = *t.into()
	}

	return tt, nil
}

func (p *SqliteSessions) DeleteAll(ctx context.Context, tt []auth.Session) error {
	if len(tt) == 0 {
		return nil
	}

	ids := make([]string, len(tt))
	for i, t := range tt {
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

type sqliteSession struct {
	ID        string    `db:"id"`
	UserID    uuid.UUID `db:"user_id"`
	ExpiresAt string    `db:"expires_at"`
}

func (t sqliteSession) into() *auth.Session {
	exp, err := time.Parse(time.RFC3339, t.ExpiresAt)
	if err != nil {
		// We serialized the string, so it should be fine
		panic(err)
	}

	return &auth.Session{
		Value:     auth.SessionID(t.ID),
		UserID:    user.ID(t.UserID),
		ExpiresAt: exp,
	}
}
