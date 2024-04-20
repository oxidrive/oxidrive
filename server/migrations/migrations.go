package migrations

import (
	"embed"
	"path"

	"github.com/oxidrive/oxidrive/server/internal/config"

	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	_ "github.com/golang-migrate/migrate/v4/database/sqlite"
	"github.com/golang-migrate/migrate/v4/source/iofs"
)

//go:embed sqlite/*.sql
//go:embed postgres/*.sql
var migrations embed.FS

func Run(cfg config.DatabaseConfig) error {
	source, err := iofs.New(migrations, path.Join(".", cfg.DatabaseName()))
	if err != nil {
		return err
	}

	m, err := migrate.NewWithSourceInstance("iofs", source, cfg.DatabaseUrl().String())
	if err != nil {
		return err
	}

	if err := m.Up(); err != nil {
		return err
	}

	srcErr, dbErr := m.Close()
	if srcErr != nil {
		return srcErr
	}
	if dbErr != nil {
		return dbErr
	}

	return nil
}
