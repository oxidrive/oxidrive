package file

import (
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func Test_NewFile(t *testing.T) {
	t.Run("creates a new valid file", func(t *testing.T) {
		t.Parallel()

		file, err := Create(nil, "this/is/a/directory/filename.txt", 5, user.ID(testutil.Must(uuid.NewV7())))

		assert.NotNil(t, file)
		assert.NoError(t, err)
	})

	t.Run("returns an error with an invalid path", func(t *testing.T) {
		t.Parallel()

		file, err := Create(nil, "/this/is/a/directory/filename.txt", 5, user.ID(testutil.Must(uuid.NewV7())))

		assert.Nil(t, file)
		assert.ErrorIs(t, err, ErrInvalidPath)
	})
}

func TestFile_isValid(t *testing.T) {
	testCases := []struct {
		testName string
		filename string
		expected bool
	}{
		{testName: "is invalid when it points to a ancestor directory", filename: "../../test.txt", expected: false},
		{testName: "is invalid when it points to a ancestor directory with complex path", filename: "this/is/the/./directory/../../../../../../test.txt", expected: false},
		{testName: "is invalid with absolute filepath", filename: "/this/is/the/directory/test.txt", expected: false},
		{testName: "is invalid when absolute filepath pointing to an ancestor", filename: "/../../../test.txt", expected: false},
		{testName: "is valid with local filepath", filename: "this/is/the/direcory/test.txt", expected: true},
	}

	for _, testCase := range testCases {
		testCase := testCase
		t.Run(testCase.testName, func(t *testing.T) {
			t.Parallel()
			require.Equal(t, testCase.expected, isValid(Path(testCase.filename)))
		})
	}
}
