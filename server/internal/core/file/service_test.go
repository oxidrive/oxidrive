package file

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func TestService_Upload(t *testing.T) {
	t.Run("uploads with valid path", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		ct := ContentType("text/plain")
		filepath := Path("/filepath")
		size := Size(10)
		owner := user.NewID()
		file := testutil.Must(Create(content, ct, filepath, size, owner))

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(nil), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.ID != file.ID && f.Path == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return f.ID != file.ID && f.Path == filepath })).Return(file, nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, ContentType: ct, Path: filepath, Size: size}

		id, err := service.Upload(ctx, toUpload, owner)

		require.NoError(t, err)
		assert.Equal(t, file.ID, id)
	})

	t.Run("uploads with invalid path", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := Path("../invalid/filepath")
		size := 10
		owner := user.NewID()

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(nil), nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)

		toUpload := FileUpload{Content: content, Path: filepath, Size: Size(size)}

		id, err := service.Upload(ctx, toUpload, owner)

		require.ErrorIs(t, err, ErrInvalidPath)
		assert.Empty(t, id)
	})

	t.Run("uploads with error while storing the content", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := Path("/filepath")
		size := 10
		genericError := errors.New("generic error")
		owner := user.NewID()

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(nil), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.Path == filepath })).Return(genericError).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: filepath, Size: Size(size)}

		id, err := service.Upload(ctx, toUpload, owner)

		require.Error(t, err)
		assert.Empty(t, id)
	})

	t.Run("uploads with error while saving metadata", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := Path("/filepath")
		size := 10
		genericError := errors.New("generic error")
		owner := user.NewID()

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(nil), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.Path == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return f.Path == filepath })).Return((*File)(nil), genericError).Once()

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		id, err := service.Upload(ctx, toUpload, owner)

		require.Error(t, err)
		assert.Empty(t, id)
	})

	t.Run("updates a file that already exists", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		ct := ContentType("text/plain")
		filepath := Path("/filepath")
		size := 10
		owner := user.NewID()

		file, err := Create(content, ct, filepath, Size(size), owner)
		require.NoError(t, err)

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(file), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.ID == file.ID && f.Path == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return f.ID == file.ID && f.Path == filepath })).Return(file, nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, ContentType: ct, Path: Path(filepath), Size: Size(size)}

		id, err := service.Upload(ctx, toUpload, owner)

		require.NoError(t, err)
		assert.Equal(t, file.ID, id)
	})
}

func TestService_Move(t *testing.T) {
	t.Run("moves a file to a valid path", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()

		owner := user.NewID()
		f, err := Create(strings.NewReader(""), "text/plain", "test.txt", 0, owner)
		require.NoError(t, err)
		require.NotNil(t, f)
		isF := mock.MatchedBy(func(fu File) bool { return fu.ID == f.ID && fu.Path == f.Path })

		newPath := Path("/hello.txt")
		u := f.Clone()
		u.Path = newPath
		isU := mock.MatchedBy(func(fu File) bool { return fu.ID == u.ID && fu.Path == u.Path })

		contentsMock.On("Copy", isF, isU).Return(nil).Once()
		contentsMock.On("Delete", isF).Return(nil).Once()
		filesMock.On("Save", isU).Return(&u, nil).Once()

		updated, err := service.Move(ctx, *f, newPath)
		require.NoError(t, err)
		require.NotNil(t, updated)

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)

		assert.Equal(t, f.ID, updated.ID)
		assert.Equal(t, newPath, updated.Path)
	})

	t.Run("recursively moves a folder and all its children", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()

		owner := user.NewID()
		newPath := Path("/renamed")

		f1, err := Create(strings.NewReader(""), "text/plain", "/folder/test.txt", 0, owner)
		require.NoError(t, err)
		require.NotNil(t, f1)
		u1 := f1.Clone()
		u1.Path = newPath + "/test.txt"
		isF1 := mock.MatchedBy(func(fu File) bool { return fu.ID == f1.ID && fu.Path == f1.Path })
		isU1 := mock.MatchedBy(func(fu File) bool { return fu.ID == u1.ID && fu.Path == u1.Path })

		f2, err := Create(strings.NewReader(""), "text/plain", "/folder/deep/test.txt", 0, owner)
		require.NoError(t, err)
		require.NotNil(t, f1)
		u2 := f2.Clone()
		u2.Path = newPath + "/deep/test.txt"
		isF2 := mock.MatchedBy(func(fu File) bool { return fu.ID == f2.ID && fu.Path == f2.Path })
		isU2 := mock.MatchedBy(func(fu File) bool { return fu.ID == u2.ID && fu.Path == u2.Path })

		d1 := File{
			ID:          NewID(),
			Type:        TypeFolder,
			ContentType: ContentTypeFolder,
			Content:     nil,
			Name:        "folder",
			Path:        "/folder",
			Size:        0,
			OwnerID:     owner,
		}
		du1 := d1.Clone()
		du1.Path = newPath
		isDU1 := mock.MatchedBy(func(fu File) bool { return fu.ID == du1.ID && fu.Path == du1.Path })

		d2 := File{
			ID:          NewID(),
			Type:        TypeFolder,
			ContentType: ContentTypeFolder,
			Content:     nil,
			Name:        "deep",
			Path:        "/folder/deep",
			Size:        0,
			OwnerID:     owner,
		}
		du2 := d2.Clone()
		du2.Path = newPath + "/deep"
		isDU2 := mock.MatchedBy(func(fu File) bool { return fu.ID == du2.ID && fu.Path == du2.Path })

		ff1 := list.Of[File]{
			Items: []File{d2, *f1},
			Count: 2,
			Total: 2,
			Next:  nil,
		}

		ff2 := list.Of[File]{
			Items: []File{*f2},
			Count: 1,
			Total: 1,
			Next:  nil,
		}

		contentsMock.On("Copy", isF1, isU1).Return(nil).Once()
		contentsMock.On("Delete", isF1).Return(nil).Once()
		contentsMock.On("Copy", isF2, isU2).Return(nil).Once()
		contentsMock.On("Delete", isF2).Return(nil).Once()
		filesMock.On("List", &d1.Path, list.NewParams()).Return(ff1, nil).Once()
		filesMock.On("List", &d2.Path, list.NewParams()).Return(ff2, nil).Once()
		filesMock.On("Save", isU1).Return(&u1, nil).Once()
		filesMock.On("Save", isU2).Return(&u2, nil).Once()
		filesMock.On("Save", isDU1).Return(&du1, nil).Once()
		filesMock.On("Save", isDU2).Return(&du2, nil).Once()

		updated, err := service.Move(ctx, d1, newPath)
		require.NoError(t, err)
		require.NotNil(t, updated)

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)

		assert.Equal(t, d1.ID, updated.ID)
		assert.Equal(t, newPath, updated.Path)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()

		id := NewID()

		filesMock.On("ByID", id).Return((*File)(nil), nil).Once()

		err := service.Delete(ctx, id)
		assert.Equal(t, err, ErrFileNotFound)

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
	})
}

func TestService_Delete(t *testing.T) {
	t.Run("deletes an existing file", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()

		owner := user.NewID()
		f, err := Create(strings.NewReader(""), "text/plain", "test.txt", 0, owner)
		require.NoError(t, err)
		require.NotNil(t, f)

		filesMock.On("ByID", f.ID).Return(f, nil).Once()
		filesMock.On("Delete", *f).Return(nil).Once()
		contentsMock.On("Delete", *f).Return(nil).Once()

		err = service.Delete(ctx, f.ID)
		require.NoError(t, err)

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
	})

	t.Run("returns an error if the file does not exist", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := NewService(contentsMock, filesMock)

		ctx := context.Background()

		id := NewID()

		filesMock.On("ByID", id).Return((*File)(nil), nil).Once()

		err := service.Delete(ctx, id)
		assert.Equal(t, err, ErrFileNotFound)

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
	})
}
