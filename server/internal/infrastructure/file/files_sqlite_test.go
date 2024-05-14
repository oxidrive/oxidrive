package file

import (
	"context"
	"strings"
	"testing"

	"github.com/jmoiron/sqlx"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	userinfra "github.com/oxidrive/oxidrive/server/internal/infrastructure/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestSqliteFiles_List(t *testing.T) {
	t.Run("returns all files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		readerMock := strings.NewReader("")

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "filepath2", 10, u.ID))))

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 2, len(ff.Items))

		assert.Equal(t, f1.ID, ff.Items[0].ID)
		assert.Equal(t, f1.Path, ff.Items[0].Path)
		assert.Equal(t, f1.Size, ff.Items[0].Size)
		assert.Equal(t, f1.OwnerID, ff.Items[0].OwnerID)

		assert.Equal(t, f2.ID, ff.Items[1].ID)
		assert.Equal(t, f2.Path, ff.Items[1].Path)
		assert.Equal(t, f2.Size, ff.Items[1].Size)
		assert.Equal(t, f2.OwnerID, ff.Items[1].OwnerID)
	})

	t.Run("returns a subset of files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		readerMock := strings.NewReader("")

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "filepath2", 10, u.ID))))

		ff, err := files.List(ctx, nil, list.Params{
			First: 1,
		})
		require.NoError(t, err)

		assert.Equal(t, 1, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Equal(t, f2.ID.String(), *ff.Next)
		require.Equal(t, 1, len(ff.Items))

		assert.Equal(t, f1.ID, ff.Items[0].ID)
		assert.Equal(t, f1.Path, ff.Items[0].Path)
		assert.Equal(t, f1.Size, ff.Items[0].Size)
		assert.Equal(t, f1.OwnerID, ff.Items[0].OwnerID)

		ff, err = files.List(ctx, nil, list.Params{
			First: 1,
			After: ff.Next,
		})
		require.NoError(t, err)

		assert.Equal(t, 1, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 1, len(ff.Items))

		assert.Equal(t, f2.ID, ff.Items[0].ID)
		assert.Equal(t, f2.Path, ff.Items[0].Path)
		assert.Equal(t, f2.Size, ff.Items[0].Size)
		assert.Equal(t, f2.OwnerID, ff.Items[0].OwnerID)
	})

	t.Run("returns no files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		files := NewSqliteFiles(db)

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 0, ff.Count)
		assert.Equal(t, 0, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 0, len(ff.Items))
	})

	t.Run("returns all files matching a prefix", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		readerMock := strings.NewReader("")

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "one/file", 10, u.ID))))
		_ = testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, "one/two/file", 10, u.ID))))

		prefix := testutil.Must(file.ParsePath("one//"))

		ff, err := files.List(ctx, &prefix, list.Params{
			First: 1,
		})
		require.NoError(t, err)

		assert.Equal(t, 1, ff.Count)
		assert.Equal(t, 1, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 1, len(ff.Items))

		assert.Equal(t, f1.ID, ff.Items[0].ID)
		assert.Equal(t, f1.Path, ff.Items[0].Path)
		assert.Equal(t, f1.Size, ff.Items[0].Size)
		assert.Equal(t, f1.OwnerID, ff.Items[0].OwnerID)
	})
}

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

func TestSqliteFiles_ByID(t *testing.T) {
	t.Run("returns an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		readerMock := strings.NewReader("")
		file, err := file.Create(readerMock, "filepath", 10, u.ID)
		require.NoError(t, err)

		file, err = files.Save(ctx, *file)
		require.NoError(t, err)

		found, err := files.ByID(ctx, file.ID)
		assert.NoError(t, err)
		assert.Equal(t, file.ID, found.ID)
		assert.Equal(t, file.Name, found.Name)
		assert.Equal(t, file.Path, found.Path)
		assert.Equal(t, file.Size, found.Size)
		assert.Equal(t, file.OwnerID, found.OwnerID)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		files := NewSqliteFiles(db)

		found, err := files.ByID(ctx, file.NewID())
		assert.NoError(t, err)
		assert.Nil(t, found)
	})
}

func TestSqliteFiles_ByOwnerByPath(t *testing.T) {
	t.Run("returns an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		readerMock := strings.NewReader("")
		file, err := file.Create(readerMock, "filepath", 10, u.ID)
		require.NoError(t, err)

		file, err = files.Save(ctx, *file)
		require.NoError(t, err)

		found, err := files.ByOwnerByPath(ctx, u.ID, file.Path)
		assert.NoError(t, err)
		assert.Equal(t, file.ID, found.ID)
		assert.Equal(t, file.Name, found.Name)
		assert.Equal(t, file.Path, found.Path)
		assert.Equal(t, file.Size, found.Size)
		assert.Equal(t, file.OwnerID, found.OwnerID)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		found, err := files.ByOwnerByPath(ctx, u.ID, "some/path")
		assert.NoError(t, err)
		assert.Nil(t, found)
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
