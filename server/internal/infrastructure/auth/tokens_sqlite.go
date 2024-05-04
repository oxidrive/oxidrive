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
	_, err := p.db.ExecContext(ctx, `insert into tokens (
        id,
        user_id,
        expires_at
    ) values (
        $1,
        $2,
        $3
    )`, t.String(), t.UserID.String(), t.ExpiresAt)
	if err != nil {
		return nil, err
	}

	return &t, nil
}

type sqliteToken struct {
	ID        string    `db:"id"`
	UserID    uuid.UUID `db:"user_id"`
	ExpiresAt time.Time `db:"expires_at"`
}

func (t sqliteToken) into() *auth.Token {
	return &auth.Token{
		Value:     auth.TokenID(t.ID),
		UserID:    user.ID(t.UserID),
		ExpiresAt: t.ExpiresAt,
	}
}
