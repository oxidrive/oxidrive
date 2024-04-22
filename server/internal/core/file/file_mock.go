package file

import (
	"context"
	"testing"

	"github.com/rs/zerolog"
	"github.com/stretchr/testify/mock"
)

type FilesContentMock struct {
	mock.Mock
}

func NewFilesContentMock(t *testing.T) *FilesContentMock {
	m := FilesContentMock{}
	m.Test(t)

	return &m
}

func (f *FilesContentMock) Store(_ context.Context, file File, logger zerolog.Logger) error {
	args := f.Called(file, logger)
	return args.Error(0)
}

type FilesMetadataMock struct {
	mock.Mock
}

func NewFilesMetadataMock(t *testing.T) *FilesMetadataMock {
	m := FilesMetadataMock{}
	m.Test(t)

	return &m
}

func (f *FilesMetadataMock) Save(_ context.Context, file File, logger zerolog.Logger) (*File, error) {
	args := f.Called(file, logger)
	return args.Get(0).(*File), args.Error(1)
}
