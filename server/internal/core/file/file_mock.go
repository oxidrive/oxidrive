package file

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"

	"github.com/oxidrive/oxidrive/server/internal/core/list"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
)

var _ Contents = (*ContentsMock)(nil)

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

var _ Files = (*FilesMock)(nil)

type FilesMock struct {
	mock.Mock
}

func NewFilesMock(t *testing.T) *FilesMock {
	m := FilesMock{}
	m.Test(t)

	return &m
}

func (f *FilesMock) List(_ context.Context, prefix *Path, params list.Params) (list.Of[File], error) {
	args := f.Called(prefix, params)
	return args.Get(0).(list.Of[File]), args.Error(1)
}

func (f *FilesMock) Save(_ context.Context, file File) (*File, error) {
	args := f.Called(file)
	return args.Get(0).(*File), args.Error(1)
}

func (f *FilesMock) ByID(_ context.Context, id ID) (*File, error) {
	args := f.Called(id)
	return args.Get(0).(*File), args.Error(1)
}

func (f *FilesMock) ByOwnerByPath(_ context.Context, owner user.ID, path Path) (*File, error) {
	args := f.Called(owner, path)
	return args.Get(0).(*File), args.Error(1)
}
