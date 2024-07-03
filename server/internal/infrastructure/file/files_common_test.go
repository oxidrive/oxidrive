package file

import (
	"context"
	"strings"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

const ct file.ContentType = file.ContentType("text/plain")

type FileTestsInit func(t *testing.T, ctx context.Context) (file.Files, user.User)

func FilesList(t *testing.T, dep testutil.IntegrationDependency, init FileTestsInit) {
	t.Run("returns all files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "filepath2", 10, u.ID))))

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Count)
		assert.Equal(t, 2, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 2, len(ff.Items))

		file.AssertEqual(t, *f1, ff.Items[0])
		file.AssertEqual(t, *f2, ff.Items[1])
	})

	t.Run("returns a subset of files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")

		f1 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "/filepath1", 10, u.ID))))
		f2 := testutil.Must(files.Save(ctx, *testutil.Must(file.Create(readerMock, ct, "/hello/filepath2", 10, u.ID))))
		d := f2.Folder()

		ff, err := files.List(ctx, nil, list.Params{
			First: 2,
		})
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Count)
		assert.Equal(t, 3, ff.Total)
		assert.NotNil(t, ff.Next)
		require.Equal(t, 2, len(ff.Items))

		file.AssertFolderEqual(t, *d, ff.Items[0])

		file.AssertEqual(t, *f1, ff.Items[1])

		ff, err = files.List(ctx, nil, list.Params{
			First: 1,
			After: ff.Next,
		})
		require.NoError(t, err)

		assert.Equal(t, 1, ff.Count)
		assert.Equal(t, 3, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 1, len(ff.Items))

		file.AssertEqual(t, *f2, ff.Items[0])
	})

	t.Run("returns no files", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, _ := init(t, ctx)

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 0, ff.Count)
		assert.Equal(t, 0, ff.Total)
		assert.Nil(t, ff.Next)
		require.Equal(t, 0, len(ff.Items))
	})

	t.Run("returns all files matching a prefix including folders", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

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

		file.AssertFolderEqual(t, *d, ff.Items[0])
		file.AssertEqual(t, *f1, ff.Items[1])
	})
}

func FilesSave(t *testing.T, dep testutil.IntegrationDependency, init FileTestsInit) {
	t.Run("saves a new file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)
		require.NoError(t, err)
		require.NotNil(t, saved)

		file.AssertEqual(t, *fileToSave, *saved)
	})

	t.Run("also saves the folder for a nested file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "/hello/world.txt", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)
		require.NoError(t, err)

		savedFolder := saved.Folder()

		file.AssertEqual(t, *fileToSave, *saved)

		ff, err := files.List(ctx, nil, list.DefaultParams)
		require.NoError(t, err)

		assert.Equal(t, 2, ff.Total)
		require.Equal(t, 2, ff.Count)

		file.AssertFolderEqual(t, *savedFolder, ff.Items[0])
		file.AssertEqual(t, *saved, ff.Items[1])
	})

	t.Run("updates the folder size when adding a new file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

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

		file.AssertFolderEqual(t, *savedFolder, ff.Items[0])
		file.AssertEqual(t, *saved1, ff.Items[1])
		file.AssertEqual(t, *saved2, ff.Items[2])
	})

	t.Run("saves an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)
		require.NoError(t, err)

		file.AssertEqual(t, *fileToSave, *saved)

		fileToSave.Name = "changed"
		fileToSave.Path = "changed"
		fileToSave.ContentType = "image/png"
		fileToSave.Size = 20

		saved, err = files.Save(ctx, *fileToSave)
		require.NoError(t, err)

		file.AssertEqual(t, *fileToSave, *saved)
	})

	t.Run("do not saves a file for a user that doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, _ := init(t, ctx)

		u, err := user.Create("missing", "password")
		require.NoError(t, err)

		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		saved, err := files.Save(ctx, *fileToSave)

		assert.Error(t, err)
		assert.Nil(t, saved)
	})

	t.Run("updates an existing folder", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		owner := u.ID

		oldPath := "/nested"
		newPath := "/updated"

		readerMock := strings.NewReader("")
		fileToSave, err := file.Create(readerMock, ct, file.Path(oldPath+"/file"), 10, owner)
		require.NoError(t, err)

		_, err = files.Save(ctx, *fileToSave)
		require.NoError(t, err)

		folder, err := files.ByOwnerByPath(ctx, owner, file.Path(oldPath))
		require.NoError(t, err)
		require.NotNil(t, folder)

		folder.Path = file.Path(newPath)

		updated, err := files.Save(ctx, *folder)
		require.NoError(t, err)

		file.AssertEqual(t, *folder, *updated)
	})
}

func FilesByID(t *testing.T, dep testutil.IntegrationDependency, init FileTestsInit) {
	t.Run("returns an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		f, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		f, err = files.Save(ctx, *f)
		require.NoError(t, err)

		found, err := files.ByID(ctx, f.ID)
		require.NoError(t, err)

		file.AssertEqual(t, *f, *found)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, _ := init(t, ctx)

		found, err := files.ByID(ctx, file.NewID())
		require.NoError(t, err)
		assert.Nil(t, found)
	})
}

func FilesByOwnerByPath(t *testing.T, dep testutil.IntegrationDependency, init FileTestsInit) {
	t.Run("returns an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		f, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		f, err = files.Save(ctx, *f)
		require.NoError(t, err)

		found, err := files.ByOwnerByPath(ctx, u.ID, f.Path)
		require.NoError(t, err)

		file.AssertEqual(t, *f, *found)
	})

	t.Run("returns nil if the file doesn't exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		found, err := files.ByOwnerByPath(ctx, u.ID, "some/path")
		require.NoError(t, err)
		assert.Nil(t, found)
	})
}

func FilesDelete(t *testing.T, dep testutil.IntegrationDependency, init FileTestsInit) {
	t.Run("deletes a file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		readerMock := strings.NewReader("")
		f, err := file.Create(readerMock, ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		f, err = files.Save(ctx, *f)
		require.NoError(t, err)
		require.NotNil(t, f)

		err = files.Delete(ctx, *f)
		require.NoError(t, err)

		f, err = files.ByID(ctx, f.ID)
		require.NoError(t, err)
		require.Nil(t, f)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, dep)
		defer done()

		files, u := init(t, ctx)

		f, err := file.Create(strings.NewReader(""), ct, "filepath", 10, u.ID)
		require.NoError(t, err)

		err = files.Delete(ctx, *f)
		assert.Equal(t, err, file.ErrFileNotFound)
	})
}
