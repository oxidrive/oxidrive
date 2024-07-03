package user

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func sqliteInit(t *testing.T, ctx context.Context) user.Users {
	db := testutil.PgDBFromContext(ctx, t)
	return NewPgUsers(db)
}

func TestSqliteUsers_Count(t *testing.T) {
	Count(t, testutil.WithPgDB(), sqliteInit)
}

func TestSqliteUsers_Save(t *testing.T) {
	Save(t, testutil.WithPgDB(), sqliteInit)
}

func TestSqliteUsers_ByID(t *testing.T) {
	ByID(t, testutil.WithPgDB(), sqliteInit)
}

func TestSqliteUsers_ByUsername(t *testing.T) {
	ByUsername(t, testutil.WithPgDB(), sqliteInit)
}
