package auth

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func pgInit(t *testing.T, ctx context.Context) auth.Tokens {
	db := testutil.PgDBFromContext(ctx, t)
	return NewPgTokens(db)
}

func TestPgTokens(t *testing.T) {
	Tokens(t, testutil.WithPgDB(), pgInit)
}
