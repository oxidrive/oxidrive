package migrations

import (
	"embed"

	"github.com/oxidrive/oxidrive/server/internal/config"

	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/source/iofs"
)

//go:embed *.sql
var migrations embed.FS

func Run(cfg config.PostgresConfig) error {
	source, err := iofs.New(migrations, ".")
	if err != nil {
		return err
	}

	m, err := migrate.NewWithSourceInstance("iofs", source, cfg.Url())
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
