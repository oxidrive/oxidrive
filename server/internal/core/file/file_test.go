package file

import (
	"testing"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"

	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
)

func Test_Create(t *testing.T) {
	testCases := []struct {
		testName         string
		filename         string
		expectedErr      error
		expectedFilepath *string
	}{
		{testName: "returns an error when the provided path points to a ancestor directory", filename: "../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path points to a ancestor directory with complex path", filename: "this/is/the/./directory/../../../../../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path is absolute", filename: "/this/is/the/directory/test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path is absolute and points to an ancestor directory", filename: "/../../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns a file when the provided path is valid", filename: "this/is/the/direcory/test.txt", expectedErr: nil, expectedFilepath: proto.String("this/is/the/direcory/test.txt")},
		{testName: "returns a file with a valid cleaned path when the provided path contains .. and .", filename: "this/is/the/direcory/../test.txt", expectedErr: nil, expectedFilepath: proto.String("this/is/the/test.txt")},
	}

	for _, testCase := range testCases {
		testCase := testCase
		t.Run(testCase.testName, func(t *testing.T) {
			t.Parallel()

			file, err := Create(nil, Path(testCase.filename), 5, user.ID(testutil.Must(uuid.NewV7())))

			if testCase.expectedFilepath != nil {
				assert.Equal(t, *testCase.expectedFilepath, string(file.Path))
			} else {
				assert.Nil(t, file)
			}
			assert.ErrorIs(t, err, testCase.expectedErr)
		})
	}

}

func Test_Update(t *testing.T) {
	testCases := []struct {
		testName         string
		filename         string
		expectedErr      error
		expectedFilepath *string
	}{
		{testName: "returns an error when the provided path points to a ancestor directory", filename: "../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path points to a ancestor directory with complex path", filename: "this/is/the/./directory/../../../../../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path is absolute", filename: "/this/is/the/directory/test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "returns an error when the provided path is absolute and points to an ancestor directory", filename: "/../../../test.txt", expectedErr: ErrInvalidPath, expectedFilepath: nil},
		{testName: "updates a file when the provided path is valid", filename: "this/is/the/direcory/test.txt", expectedErr: nil, expectedFilepath: proto.String("this/is/the/direcory/test.txt")},
		{testName: "updates a file with a valid cleaned path when the provided path contains .. and .", filename: "this/is/the/direcory/../test.txt", expectedErr: nil, expectedFilepath: proto.String("this/is/the/test.txt")},
	}

	originalPath := Path("valid/filename.txt")

	for _, testCase := range testCases {
		testCase := testCase
		t.Run(testCase.testName, func(t *testing.T) {
			t.Parallel()

			file, err := Create(nil, originalPath, 5, user.ID(testutil.Must(uuid.NewV7())))
			require.NoError(t, err)

			err = file.Update(nil, Path(testCase.filename), 5)

			if testCase.expectedFilepath != nil {
				assert.Equal(t, *testCase.expectedFilepath, string(file.Path))
			} else {
				assert.Equal(t, originalPath, file.Path)
			}
			assert.ErrorIs(t, err, testCase.expectedErr)
		})
	}
}
