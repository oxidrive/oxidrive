package file

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

type ContentsMock struct {
	mock.Mock
}

func NewContentsMock(t *testing.T) *ContentsMock {
	m := ContentsMock{}
	m.Test(t)

	return &m
}

func (c *ContentsMock) Store(_ context.Context, file File) error {
	args := c.Called(file)
	return args.Error(0)
}

type FilesMock struct {
	mock.Mock
}

func NewFilesMock(t *testing.T) *FilesMock {
	m := FilesMock{}
	m.Test(t)

	return &m
}

func (f *FilesMock) Save(_ context.Context, file File) (*File, error) {
	args := f.Called(file)
	return args.Get(0).(*File), args.Error(1)
}

func (f *FilesMock) ByOwnerByPath(_ context.Context, owner user.ID, path Path) (*File, error) {
	args := f.Called(owner, path)
	return args.Get(0).(*File), args.Error(1)
}
