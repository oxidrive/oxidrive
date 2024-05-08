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

var _ auth.Tokens = (*PgTokens)(nil)

type SqliteTokens struct {
	db *sqlx.DB
}

func NewSqliteTokens(db *sqlx.DB) *SqliteTokens {
	return &SqliteTokens{db: db}
}

func (p *SqliteTokens) ByID(ctx context.Context, id auth.TokenID) (*auth.Token, error) {
	var t sqliteToken
	err := p.db.GetContext(ctx, &t, "select id, user_id, expires_at from tokens where id = $1", string(id))
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	return t.into(), nil
}

func (p *SqliteTokens) Store(ctx context.Context, t auth.Token) (*auth.Token, error) {
	exp := t.ExpiresAt.UTC().Format(time.RFC3339)
	_, err := p.db.ExecContext(ctx, `insert into tokens (
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

func (p *SqliteTokens) ExpiringBefore(ctx context.Context, exp time.Time) ([]auth.Token, error) {
	e := exp.UTC().Format(time.RFC3339)
	stt := make([]sqliteToken, 0)
	err := p.db.SelectContext(ctx, &stt, "select id, user_id, expires_at from tokens where expires_at <= $1", e)
	if errors.Is(err, sql.ErrNoRows) {
		return []auth.Token{}, nil
	}

	if err != nil {
		return nil, err
	}

	tt := make([]auth.Token, len(stt))

	for i, t := range stt {
		tt[i] = *t.into()
	}

	return tt, nil
}

func (p *SqliteTokens) DeleteAll(ctx context.Context, tt []auth.Token) error {
	if len(tt) == 0 {
		return nil
	}

	ids := make([]string, len(tt))
	for i, t := range tt {
		ids[i] = t.Value.String()
	}

	query, args, err := sqlx.In("delete from tokens where id in (?)", ids)
	if err != nil {
		return err
	}

	query = p.db.Rebind(query)

	_, err = p.db.ExecContext(ctx, query, args...)
	return err
}

type sqliteToken struct {
	ID        string    `db:"id"`
	UserID    uuid.UUID `db:"user_id"`
	ExpiresAt string    `db:"expires_at"`
}

func (t sqliteToken) into() *auth.Token {
	exp, err := time.Parse(time.RFC3339, t.ExpiresAt)
	if err != nil {
		// We serialized the string, so it should be fine
		panic(err)
	}

	return &auth.Token{
		Value:     auth.TokenID(t.ID),
		UserID:    user.ID(t.UserID),
		ExpiresAt: exp,
	}
}
