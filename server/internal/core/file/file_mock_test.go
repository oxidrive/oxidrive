package file

import (
	"context"
	"errors"
	"testing"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestFilesContentMock_Store(t *testing.T) {
	filesMock := NewFilesContentMock(t)
	file, _ := NewFile(nil, "namefile.txt", 10)

	t.Run("with errors", func(t *testing.T) {
		mockError := errors.New("error while storeing new file")
		filesMock.On("Store", *file, zerolog.Nop()).Return(mockError).Once()
		defer filesMock.AssertExpectations(t)

		err := filesMock.Store(context.Background(), *file, zerolog.Nop())

		require.ErrorIs(t, err, mockError)
	})

	t.Run("without errors", func(t *testing.T) {
		filesMock.On("Store", *file, zerolog.Nop()).Return(nil).Once()
		defer filesMock.AssertExpectations(t)

		err := filesMock.Store(context.Background(), *file, zerolog.Nop())

		require.NoError(t, err)
	})
}

func TestFilesMetadataMock_Save(t *testing.T) {
	filesMock := NewFilesMetadataMock(t)
	file, _ := NewFile(nil, "namefile.txt", 10)

	t.Run("with errors", func(t *testing.T) {
		mockError := errors.New("error while saving new file")
		filesMock.On("Save", *file, zerolog.Nop()).Return((*File)(nil), mockError).Once()
		defer filesMock.AssertExpectations(t)

		savedFile, err := filesMock.Save(context.Background(), *file, zerolog.Nop())

		assert.Nil(t, savedFile)
		assert.ErrorIs(t, err, mockError)
	})

	t.Run("without errors", func(t *testing.T) {
		filesMock.On("Save", *file, zerolog.Nop()).Return(file, nil).Once()
		defer filesMock.AssertExpectations(t)

		savedFile, err := filesMock.Save(context.Background(), *file, zerolog.Nop())

		assert.NotNil(t, savedFile)
		assert.NoError(t, err)
	})
}
