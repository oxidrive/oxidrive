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

func sqliteInit(t *testing.T, ctx context.Context) (file.Files, user.User) {
	db := testutil.SqliteDBFromContext(ctx, t)
	u := insertSqliteUser(t, db, "username", "pwd")

	return NewSqliteFiles(db), u
}

func TestSqliteFiles_List(t *testing.T) {
	List(t, testutil.WithSqliteDB(), sqliteInit)
}

func TestSqliteFiles_Save(t *testing.T) {
	Save(t, testutil.WithSqliteDB(), sqliteInit)
}

func TestSqliteFiles_ByID(t *testing.T) {
	ByID(t, testutil.WithSqliteDB(), sqliteInit)
}

func TestSqliteFiles_ByOwnerByPath(t *testing.T) {
	ByOwnerByPath(t, testutil.WithSqliteDB(), sqliteInit)
}

func TestSqliteFiles_Delete(t *testing.T) {
	Delete(t, testutil.WithSqliteDB(), sqliteInit)
}

func insertSqliteUser(t *testing.T, db *sqlx.DB, username string, password string) user.User {
	t.Helper()

	users := userinfra.NewSqliteUsers(db)
	u := testutil.Must(user.Create(username, password))

	if _, err := users.Save(context.Background(), *u); err != nil {
		t.Fatal(err)
	}

	return *u
}
