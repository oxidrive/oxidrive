package file

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"

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

		file, err := Create(content, ct, "original", Size(size), owner)
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
