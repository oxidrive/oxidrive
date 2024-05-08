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

type PgTokens struct {
	db *sqlx.DB
}

func NewPgTokens(db *sqlx.DB) *PgTokens {
	return &PgTokens{db: db}
}

func (p *PgTokens) ByID(ctx context.Context, id auth.TokenID) (*auth.Token, error) {
	var t pgToken
	err := p.db.GetContext(ctx, &t, "select id, user_id, expires_at from tokens where id = $1", string(id))
	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	}

	if err != nil {
		return nil, err
	}

	return t.into(), nil
}

func (p *PgTokens) Store(ctx context.Context, t auth.Token) (*auth.Token, error) {
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

func (p *PgTokens) ExpiringBefore(ctx context.Context, exp time.Time) ([]auth.Token, error) {
	ptt := make([]pgToken, 0)
	err := p.db.SelectContext(ctx, &ptt, "select id, user_id, expires_at from tokens where expires_at <= $1", exp)
	if errors.Is(err, sql.ErrNoRows) {
		return []auth.Token{}, nil
	}

	if err != nil {
		return nil, err
	}

	tt := make([]auth.Token, len(ptt))

	for i, t := range ptt {
		tt[i] = *t.into()
	}

	return tt, nil
}

func (p *PgTokens) DeleteAll(ctx context.Context, tt []auth.Token) error {
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

type pgToken struct {
	ID        string    `db:"id"`
	UserID    uuid.UUID `db:"user_id"`
	ExpiresAt time.Time `db:"expires_at"`
}

func (t pgToken) into() *auth.Token {
	return &auth.Token{
		Value:     auth.TokenID(t.ID),
		UserID:    user.ID(t.UserID),
		ExpiresAt: t.ExpiresAt,
	}
}
