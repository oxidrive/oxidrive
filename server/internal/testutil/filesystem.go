package testutil

import (
	"context"
	"os"
	"testing"
)

// FileSystemContainerConfig holds all configs for the temp filesystem that can be used as a dependency for integration tests
type FileSystemContainerConfig struct {
	Path string // Path where the temporary directory is created. Note that it will be deleted at the end of the test.
}

// WithFilesystem creates a tempdir using the std testing lib
func WithFilesystem(cfg FileSystemContainerConfig) func(context.Context, *testing.T) func() {
	return func(_ context.Context, t *testing.T) func() {
		t.Helper()

		if err := os.MkdirAll(cfg.Path, 0750); err != nil {
			t.Fatal(err)
		}

		return func() {
			if err := os.RemoveAll(cfg.Path); err != nil {
				t.Fatal(err)
			}
		}
	}
}
