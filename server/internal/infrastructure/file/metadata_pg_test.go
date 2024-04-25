package file

import (
	"context"
	"strings"
	"testing"

	"github.com/rs/zerolog"

	"github.com/oxidrive/oxidrive/server/internal/core/file"
	"github.com/oxidrive/oxidrive/server/internal/testutil"

	"github.com/stretchr/testify/assert"
)

func TestPgFiles_Save(t *testing.T) {
	t.Run("saves a new file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		files := NewPgFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, _ := file.NewFile(readerMock, "filepath", 10)

		saveed, err := files.Save(ctx, *fileToSave, zerolog.Nop())

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saveed.Name)
		assert.Equal(t, fileToSave.Path, saveed.Path)
		assert.Equal(t, fileToSave.Size, saveed.Size)
	})

	t.Run("saves an existing file", func(t *testing.T) {
		t.Parallel()

		ctx, done := testutil.IntegrationTest(context.Background(), t, testutil.WithPgDB())
		defer done()

		db := testutil.PgDBFromContext(ctx, t)

		files := NewPgFiles(db)
		readerMock := strings.NewReader("")
		fileToSave, _ := file.NewFile(readerMock, "filepath", 10)

		saveed, err := files.Save(ctx, *fileToSave, zerolog.Nop())

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saveed.Name)
		assert.Equal(t, fileToSave.Path, saveed.Path)
		assert.Equal(t, fileToSave.Size, saveed.Size)

		fileToSave.Name = "changed"
		fileToSave.Path = "changed"
		fileToSave.Size = 20

		saveed, err = files.Save(ctx, *fileToSave, zerolog.Nop())

		assert.NoError(t, err)
		assert.Equal(t, fileToSave.Name, saveed.Name)
		assert.Equal(t, fileToSave.Path, saveed.Path)
		assert.Equal(t, fileToSave.Size, saveed.Size)
	})
}
