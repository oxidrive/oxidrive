package file

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func AssertEqual(t *testing.T, expected File, actual File) {
	assert.Equal(t, expected.Type, actual.Type)
	assert.Equal(t, expected.ID, actual.ID)
	assert.Equal(t, expected.Name, actual.Name)
	assert.Equal(t, expected.Path, actual.Path)
	assert.Equal(t, expected.ContentType, actual.ContentType)
	assert.Equal(t, expected.Size, actual.Size)
	assert.Equal(t, expected.OwnerID, actual.OwnerID)
}

func AssertFolderEqual(t *testing.T, expected Folder, actual File) {
	assert.Equal(t, TypeFolder, actual.Type)
	assert.Equal(t, expected.Name, actual.Name)
	assert.Equal(t, expected.Path, actual.Path)
}
