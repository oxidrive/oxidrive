package file

import (
	"context"
	"testing"

	"github.com/jmoiron/sqlx"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func pgInit(t *testing.T, ctx context.Context) (file.Files, user.User) {
	db := testutil.PgDBFromContext(ctx, t)
	u := insertPgUser(t, db, "username", "pwd")

	return NewPgFiles(db), u
}

func TestPgFiles_List(t *testing.T) {
	FilesList(t, testutil.WithPgDB(), pgInit)
}

func TestPgFiles_Save(t *testing.T) {
	FilesSave(t, testutil.WithPgDB(), pgInit)
}

func TestPgFiles_ByID(t *testing.T) {
	FilesByID(t, testutil.WithPgDB(), pgInit)
}

func TestPgFiles_ByOwnerByPath(t *testing.T) {
	FilesByOwnerByPath(t, testutil.WithPgDB(), pgInit)
}

func TestPgFiles_Delete(t *testing.T) {
	FilesDelete(t, testutil.WithPgDB(), pgInit)
}

func insertPgUser(t *testing.T, db *sqlx.DB, username string, password string) user.User {
	t.Helper()

	users := userinfra.NewPgUsers(db)
	u := testutil.Must(user.Create(username, password))

	if _, err := users.Save(context.Background(), *u); err != nil {
		t.Fatal(err)
	}

	return *u
}
