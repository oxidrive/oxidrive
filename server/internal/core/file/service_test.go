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
	contentMock := NewFilesContentMock(t)
	metadataMock := NewFilesMetadataMock(t)

	service := InitService(contentMock, metadataMock)

	t.Run("uploads with valid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10

		contentMock.On(
			"Store",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
		).Return(nil).Once()
		metadataMock.On(
			"Save",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
		).Return((*File)(nil), nil).Once()
		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), user.ID(testutil.Must(uuid.NewV7())))

		require.NoError(t, err)
	})

	t.Run("uploads with invalid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "/abs/filepath"
		size := 10

		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), user.ID(testutil.Must(uuid.NewV7())))

		require.ErrorIs(t, err, ErrInvalidPath)
	})

	t.Run("uploads with error while storing the content", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		genericError := errors.New("generic error")

		contentMock.On(
			"Store",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
		).Return(genericError).Once()
		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), user.ID(testutil.Must(uuid.NewV7())))

		require.Error(t, err)
	})

	t.Run("uploads with error while saving metadata", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		genericError := errors.New("generic error")

		contentMock.On(
			"Store",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
		).Return(nil).Once()
		metadataMock.On(
			"Save",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
		).Return((*File)(nil), genericError).Once()

		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), user.ID(testutil.Must(uuid.NewV7())))

		require.Error(t, err)
	})
}
