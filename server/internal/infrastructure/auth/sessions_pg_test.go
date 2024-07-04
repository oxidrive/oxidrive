package auth

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func pgInit(t *testing.T, ctx context.Context) auth.Sessions {
	db := testutil.PgDBFromContext(ctx, t)
	return NewPgSessions(db)
}

func TestPgSessions(t *testing.T) {
	Sessions(t, testutil.WithPgDB(), pgInit)
}
