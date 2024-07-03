package auth

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func sqliteInit(t *testing.T, ctx context.Context) auth.Tokens {
	db := testutil.SqliteDBFromContext(ctx, t)
	return NewSqliteTokens(db)
}

func TestSqliteTokens(t *testing.T) {
	Tokens(t, testutil.WithSqliteDB(), sqliteInit)
}
