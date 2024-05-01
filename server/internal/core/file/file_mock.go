package file

import (
	"context"
	"testing"

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

func (f *FilesContentMock) Store(_ context.Context, file File) error {
	args := f.Called(file)
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

func (f *FilesMetadataMock) Save(_ context.Context, file File) (*File, error) {
	args := f.Called(file)
	return args.Get(0).(*File), args.Error(1)
}
