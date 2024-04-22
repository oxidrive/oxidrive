package file

import (
	"context"
	"strings"
	"testing"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/mock"
	"github.com/stretchr/testify/require"
)

func TestService_Upload(t *testing.T) {
	t.Parallel()
	contentMock := NewFilesContentMock(t)
	metadataMock := NewFilesMetadataMock(t)

	service := NewService(contentMock, metadataMock)

	t.Run("uplaods with valid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "filepath"
		size := 10
		expectedLogger := zerolog.Nop().With().Str("path", filepath).Int("size", size).Logger()

		contentMock.On("Store", mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }), expectedLogger).Return(nil).Once()
		metadataMock.On(
			"Save",
			mock.MatchedBy(func(f File) bool { return string(f.Path) == filepath }),
			expectedLogger,
		).Return((*File)(nil), nil).Once()
		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), zerolog.Nop())

		require.NoError(t, err)
	})

	t.Run("uplaods with invalid path", func(t *testing.T) {
		ctx := context.Background()
		content := strings.NewReader("")
		filepath := "/abs/filepath"
		size := 10

		defer contentMock.AssertExpectations(t)
		defer metadataMock.AssertExpectations(t)

		err := service.Upload(ctx, content, Path(filepath), Size(size), zerolog.Nop())

		require.ErrorIs(t, err, ErrInvalidPath)
	})
}
