package file

import (
	"context"
	"strings"
	"testing"

	"github.com/jmoiron/sqlx"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestSqliteFiles_Save(t *testing.T) {
	t.Run("saves a new file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.Size, saved.Size)
	})

	t.Run("saves an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.Size, saved.Size)

		fileToSave.Name = "changed"
		fileToSave.Path = "changed"
		fileToSave.Size = 20

		saved, err = files.Save(ctx, *fileToSave)

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.Size, saved.Size)
	})

	t.Run("do not saves with a not existing user", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u, _ := user.Create("username", "password")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		assert.Error(t, err)
		assert.Nil(t, saved)
	})

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
