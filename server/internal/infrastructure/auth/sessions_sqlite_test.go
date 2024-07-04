package auth

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/auth"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func sqliteInit(t *testing.T, ctx context.Context) auth.Sessions {
	db := testutil.SqliteDBFromContext(ctx, t)
	return NewSqliteSessions(db)
}

func TestSqliteSessions(t *testing.T) {
	Sessions(t, testutil.WithSqliteDB(), sqliteInit)
}
