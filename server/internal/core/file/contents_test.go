package file

import (
	"context"
	"testing"

	"github.com/stretchr/testify/mock"
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

func (c *ContentsMock) Load(_ context.Context, file File) (Content, error) {
	args := c.Called(file)
	return args.Get(0).(Content), args.Error(1)
}

func (c *ContentsMock) Delete(_ context.Context, file File) error {
	args := c.Called(file)
	return args.Error(0)
}

func (c *ContentsMock) Copy(ctx context.Context, from File, to File) error {
	args := c.Called(from, to)
	return args.Error(0)
}
