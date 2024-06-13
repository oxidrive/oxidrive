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

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath2", 10, u.ID))))

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 2, len(ff.Items))

		assert.Equal(t, f1.ID, ff.Items[0].ID)
		assert.Equal(t, file.TypeFile, ff.Items[0].Type)
		assert.Equal(t, f1.Path, ff.Items[0].Path)
		assert.Equal(t, f1.ContentType, ff.Items[0].ContentType)
		assert.Equal(t, f1.Size, ff.Items[0].Size)
		assert.Equal(t, f1.OwnerID, ff.Items[0].OwnerID)

		assert.Equal(t, f2.ID, ff.Items[1].ID)
		assert.Equal(t, file.TypeFile, ff.Items[1].Type)
		assert.Equal(t, f2.Path, ff.Items[1].Path)
		assert.Equal(t, f2.ContentType, ff.Items[1].ContentType)
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

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath2", 10, u.ID))))

		ff, err := files.List(ctx, nil, list.Params{
			First: 1,
		})
		require.NoError(t, err)

		assert.Equal(t, 1, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.NotNil(t, ff.Next)
		require.Equal(t, 1, len(ff.Items))

		assert.Equal(t, f1.ID, ff.Items[0].ID)
		assert.Equal(t, file.TypeFile, ff.Items[0].Type)
		assert.Equal(t, f1.Path, ff.Items[0].Path)
		assert.Equal(t, f1.ContentType, ff.Items[0].ContentType)
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
		assert.Equal(t, file.TypeFile, ff.Items[0].Type)
		assert.Equal(t, f2.Path, ff.Items[0].Path)
		assert.Equal(t, f2.ContentType, ff.Items[0].ContentType)
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

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "one/file", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "one/two/file", 10, u.ID))))

		d := f2.Folder()

		prefix := testutil.Must(file.ParsePath("one//"))

		ff, err := files.List(ctx, &prefix, list.Params{
			First: 2,
		})
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 2, len(ff.Items))

		assert.Equal(t, d.Name, ff.Items[0].Name)
		assert.Equal(t, file.TypeFolder, ff.Items[0].Type)
		assert.Equal(t, d.Path, ff.Items[0].Path)
		assert.Equal(t, f2.Size, ff.Items[0].Size)
		assert.Equal(t, f2.OwnerID, ff.Items[0].OwnerID)

		assert.Equal(t, f1.ID, ff.Items[1].ID)
		assert.Equal(t, file.TypeFile, ff.Items[1].Type)
		assert.Equal(t, f1.Name, ff.Items[1].Name)
		assert.Equal(t, f1.ContentType, ff.Items[1].ContentType)
		assert.Equal(t, f1.Path, ff.Items[1].Path)
		assert.Equal(t, f1.Size, ff.Items[1].Size)
		assert.Equal(t, f1.OwnerID, ff.Items[1].OwnerID)
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
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		require.NoError(t, err)
		assert.Equal(t, file.TypeFile, saved.Type)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.ContentType, saved.ContentType)
		assert.Equal(t, fileToSave.Size, saved.Size)
	})

	t.Run("also saves the folder for a nested file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "/hello/world.txt", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)
		require.NoError(t, err)

		savedFolder := saved.Folder()

		assert.Equal(t, file.TypeFile, saved.Type)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.ContentType, saved.ContentType)
		assert.Equal(t, fileToSave.Size, saved.Size)

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Total)
		require.Equal(t, 2, ff.Count)

		d := ff.Items[0]
		assert.Equal(t, file.TypeFolder, d.Type)
		assert.Equal(t, savedFolder.Name, d.Name)
		assert.Equal(t, savedFolder.Path, d.Path)
		assert.Equal(t, saved.Size, d.Size)

		f := ff.Items[1]
		assert.Equal(t, saved.ID, f.ID)
		assert.Equal(t, file.TypeFile, f.Type)
		assert.Equal(t, saved.Name, f.Name)
		assert.Equal(t, saved.Path, f.Path)
		assert.Equal(t, saved.ContentType, f.ContentType)
		assert.Equal(t, saved.Size, f.Size)
	})

	t.Run("updates the folder size when adding a new file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		file1, err := file.Create(readerMock, ct, "/hello/one.txt", 10, u.ID)
		require.NoError(t, err)

		saved1, err := files.Save(ctx, *file1)
		require.NoError(t, err)

		file2, err := file.Create(readerMock, ct, "/hello/world.txt", 32, u.ID)
		require.NoError(t, err)

		saved2, err := files.Save(ctx, *file2)
		require.NoError(t, err)

		assert.Equal(t, saved1.Folder(), saved2.Folder())

		savedFolder := saved1.Folder()

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 3, ff.Total)
		require.Equal(t, 3, ff.Count)

		d := ff.Items[0]
		assert.Equal(t, file.TypeFolder, d.Type)
		assert.Equal(t, savedFolder.Name, d.Name)
		assert.Equal(t, savedFolder.Path, d.Path)
		assert.Equal(t, saved1.Size+saved2.Size, d.Size)

		f1 := ff.Items[1]
		assert.Equal(t, saved1.ID, f1.ID)
		assert.Equal(t, file.TypeFile, f1.Type)
		assert.Equal(t, saved1.Name, f1.Name)
		assert.Equal(t, saved1.Path, f1.Path)
		assert.Equal(t, saved1.ContentType, f1.ContentType)
		assert.Equal(t, saved1.Size, f1.Size)

		f2 := ff.Items[2]
		assert.Equal(t, saved2.ID, f2.ID)
		assert.Equal(t, file.TypeFile, f2.Type)
		assert.Equal(t, saved2.Name, f2.Name)
		assert.Equal(t, saved2.Path, f2.Path)
		assert.Equal(t, saved2.ContentType, f2.ContentType)
		assert.Equal(t, saved2.Size, f2.Size)
	})

	t.Run("saves an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		require.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, file.TypeFile, saved.Type)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.ContentType, saved.ContentType)
		assert.Equal(t, fileToSave.Size, saved.Size)

		fileToSave.Name = "changed"
		fileToSave.Path = "changed"
		fileToSave.ContentType = "image/png"
		fileToSave.Size = 20

		saved, err = files.Save(ctx, *fileToSave)

		require.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saved.Name)
		assert.Equal(t, file.TypeFile, saved.Type)
		assert.Equal(t, fileToSave.Path, saved.Path)
		assert.Equal(t, fileToSave.ContentType, saved.ContentType)
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
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
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
		f, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		f, err = files.Save(ctx, *f)
		require.NoError(t, err)

		found, err := files.ByID(ctx, f.ID)
		require.NoError(t, err)
		assert.Equal(t, f.ID, found.ID)
		assert.Equal(t, file.TypeFile, found.Type)
		assert.Equal(t, f.Name, found.Name)
		assert.Equal(t, f.Path, found.Path)
		assert.Equal(t, f.ContentType, found.ContentType)
		assert.Equal(t, f.Size, found.Size)
		assert.Equal(t, f.OwnerID, found.OwnerID)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)

		files := NewSqliteFiles(db)

		found, err := files.ByID(ctx, file.NewID())
		require.NoError(t, err)
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
		f, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		f, err = files.Save(ctx, *f)
		require.NoError(t, err)

		found, err := files.ByOwnerByPath(ctx, u.ID, f.Path)
		require.NoError(t, err)
		assert.Equal(t, f.ID, found.ID)
		assert.Equal(t, file.TypeFile, found.Type)
		assert.Equal(t, f.Name, found.Name)
		assert.Equal(t, f.Path, found.Path)
		assert.Equal(t, f.Size, found.Size)
		assert.Equal(t, f.OwnerID, found.OwnerID)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithSqliteDB(testutil.SqliteDBConfig{DbParams: "_foreign_keys=on"}))
		defer done()

		db := testutil.SqliteDBFromContext(ctx, t)
		u := insertSqliteUser(t, db, "username", "pwd")

		files := NewSqliteFiles(db)

		found, err := files.ByOwnerByPath(ctx, u.ID, "some/path")
		require.NoError(t, err)
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
