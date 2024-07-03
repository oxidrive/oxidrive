package user

import (
	"context"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func pgInit(t *testing.T, ctx context.Context) user.Users {
	db := testutil.PgDBFromContext(ctx, t)
	return NewPgUsers(db)
}

func TestPgUsers_Count(t *testing.T) {
	Count(t, testutil.WithPgDB(), pgInit)
}

func TestPgUsers_Save(t *testing.T) {
	Save(t, testutil.WithPgDB(), pgInit)
}

func TestPgUsers_ByID(t *testing.T) {
	ByID(t, testutil.WithPgDB(), pgInit)
}

func TestPgUsers_ByUsername(t *testing.T) {
	ByUsername(t, testutil.WithPgDB(), pgInit)
}
