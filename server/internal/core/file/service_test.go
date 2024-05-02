package file

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/google/uuid"
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

		service := InitService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10

		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath })).Return((*File)(nil), nil).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		err := service.Upload(ctx, toUpload, user.ID(testutil.Must(uuid.NewV7())))

		require.NoError(t, err)
	})

	t.Run("uploads with invalid path", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := InitService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "/abs/filepath"
		size := 10

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		err := service.Upload(ctx, toUpload, user.ID(testutil.Must(uuid.NewV7())))

		require.ErrorIs(t, err, ErrInvalidPath)
	})

	t.Run("uploads with error while storing the content", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := InitService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		genericError := errors.New("generic error")

		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath })).Return(genericError).Once()
		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		err := service.Upload(ctx, toUpload, user.ID(testutil.Must(uuid.NewV7())))

		require.Error(t, err)
	})

	t.Run("uploads with error while saving metadata", func(t *testing.T) {
		t.Parallel()

		contentsMock := NewContentsMock(t)
		filesMock := NewFilesMock(t)

		service := InitService(contentsMock, filesMock)

		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		genericError := errors.New("generic error")

		contentsMock.On("Store", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath })).Return(nil).Once()
		filesMock.On("Save", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath })).Return((*File)(nil), genericError).Once()

		defer contentsMock.AssertExpectations(t)
		defer filesMock.AssertExpectations(t)
		toUpload := FileUpload{Content: content, Path: Path(filepath), Size: Size(size)}

		err := service.Upload(ctx, toUpload, user.ID(testutil.Must(uuid.NewV7())))

		require.Error(t, err)
	})
}
