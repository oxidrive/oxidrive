package user

import (
	"fmt"
	"net/url"
	"path"
	"testing"

	"github.com/jmoiron/sqlx"
	"github.com/oxidrive/oxidrive/server/internal/config"
	"github.com/oxidrive/oxidrive/server/internal/core/user"
	"github.com/oxidrive/oxidrive/server/internal/testutil"
	"github.com/oxidrive/oxidrive/server/migrations"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestSqliteUsers_Count(t *testing.T) {
	testutil.IntegrationTest(t)

	t.Run("returns the number of users", func(t *testing.T) {
		dir := t.TempDir()
		url, err := url.Parse(fmt.Sprintf("sqlite://%s", path.Join(dir, "oxidrive.db")))
		assert.NoError(t, err)
		cfg := config.DatabaseConfig{Url: url}

		assert.NoError(t, migrations.Run(cfg))

		db, err := sqlx.Connect(cfg.DatabaseDriver(), cfg.DatabaseSource())
		assert.NoError(t, err)

		users := NewSqliteUsers(db)

		userMust(users.Save(userMust(user.Create("a", "a"))))
		userMust(users.Save(userMust(user.Create("b", "b"))))

		count, err := users.Count()

		assert.NoError(t, err)
		assert.Equal(t, 2, count)
	})
}

func TestSqliteUsers_Save(t *testing.T) {
	testutil.IntegrationTest(t)

	t.Run("creates a new user", func(t *testing.T) {
		dir := t.TempDir()
		url, err := url.Parse(fmt.Sprintf("sqlite://%s", path.Join(dir, "oxidrive.db")))
		assert.NoError(t, err)
		cfg := config.DatabaseConfig{Url: url}

		assert.NoError(t, migrations.Run(cfg))

		db, err := sqlx.Connect(cfg.DatabaseDriver(), cfg.DatabaseSource())
		assert.NoError(t, err)

		username := "testuser"

		users := NewSqliteUsers(db)

		created, err := users.Save(userMust(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)
	})

	t.Run("updates an existing user", func(t *testing.T) {
		dir := t.TempDir()
		url, err := url.Parse(fmt.Sprintf("sqlite://%s", path.Join(dir, "oxidrive.db")))
		assert.NoError(t, err)
		cfg := config.DatabaseConfig{Url: url}

		assert.NoError(t, migrations.Run(cfg))

		db, err := sqlx.Connect(cfg.DatabaseDriver(), cfg.DatabaseSource())
		assert.NoError(t, err)

		username := "testuser"

		users := NewSqliteUsers(db)

		created, err := users.Save(userMust(user.Create(username, "a")))
		assert.NoError(t, err)
		assert.Equal(t, username, created.Username)

		changedUsername := "changed"
		created.Username = changedUsername

		updated, err := users.Save(created)
		assert.NoError(t, err)
		assert.Equal(t, created.Id, updated.Id)
		assert.Equal(t, changedUsername, updated.Username)
	})
}

func userMust(u user.User, err error) user.User {
	if err != nil {
		panic(err)
	}
	return u
}
