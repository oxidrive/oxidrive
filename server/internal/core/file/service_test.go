package file

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/google/uuid"
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
		filepath := Path("filepath")
		size := Size(10)
		owner := user.ID(testutil.Must(uuid.NewV7()))
		file := testutil.Must(Create(content, filepath, size, owner))

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(nil), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.ID != file.ID && f.Path == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return f.ID != file.ID && f.Path == filepath })).Return(file, nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: filepath, Size: size}

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
		owner := user.ID(testutil.Must(uuid.NewV7()))

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
		filepath := Path("filepath")
		size := 10
		genericError := errors.New("generic error")
		owner := user.ID(testutil.Must(uuid.NewV7()))

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
		filepath := Path("filepath")
		size := 10
		genericError := errors.New("generic error")
		owner := user.ID(testutil.Must(uuid.NewV7()))

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
		filepath := Path("filepath")
		size := 10
		owner := user.ID(testutil.Must(uuid.NewV7()))

		file, err := Create(content, "original", Size(size), owner)
		require.NoError(t, err)

		filesMock.On("ByOwnerByPath", owner, filepath).Return((*File)(file), nil).Once()
		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return f.ID == file.ID && f.Path == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return f.ID == file.ID && f.Path == filepath })).Return(file, nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		id, err := service.Upload(ctx, toUpload, owner)

		require.NoError(t, err)
		assert.Equal(t, file.ID, id)
	})
}
