package file

import (
	"context"
	"errors"
	"strings"
	"testing"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/rs/zerolog"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestService_Upload(t *testing.T) {
	contentMock := NewFilesContentMock(t)
	metadataMock := NewFilesMetadataMock(t)

	service := NewService(contentMock, metadataMock)
	createdUser, _ := user.Create("username", "password")

	t.Run("uploads with valid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		expectedLogger := zerolog.Nop().With().Str("path", filepath).Int("size", size).Logger()

		contentMock.On(
			"Store",
			mock.MatchedBy(func(u user.User) bool { return string(u.Username) == "username" }),
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return(nil).Once()
		metadataMock.On(
			"Save",
			mock.MatchedBy(func(u user.User) bool { return string(u.Username) == "username" }),
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return((*File)(nil), nil).Once()
		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, *createdUser, content, Path(filepath), Size(size), zerolog.Nop())

		require.NoError(t, err)
	})

	t.Run("uploads with invalid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "/abs/filepath"
		size := 10

		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, *createdUser, content, Path(filepath), Size(size), zerolog.Nop())

		require.ErrorIs(t, err, ErrInvalidPath)
	})

	t.Run("uploads with error while storing the content", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		expectedLogger := zerolog.Nop().With().Str("path", filepath).Int("size", size).Logger()
		genericError := errors.New("generic error")

		contentMock.On(
			"Store",
			mock.MatchedBy(func(u user.User) bool { return string(u.Username) == "username" }),
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return(genericError).Once()
		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, *createdUser, content, Path(filepath), Size(size), zerolog.Nop())

		require.Error(t, err)
	})

	t.Run("uploads with error while saving metadata", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		expectedLogger := zerolog.Nop().With().Str("path", filepath).Int("size", size).Logger()
		genericError := errors.New("generic error")

		contentMock.On(
			"Store",
			mock.MatchedBy(func(u user.User) bool { return string(u.Username) == "username" }),
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return(nil).Once()
		metadataMock.On(
			"Save",
			mock.MatchedBy(func(u user.User) bool { return string(u.Username) == "username" }),
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return((*File)(nil), genericError).Once()

		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, *createdUser, content, Path(filepath), Size(size), zerolog.Nop())

		require.Error(t, err)
	})
}
