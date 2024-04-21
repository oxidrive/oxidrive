package config

import (
	"fmt"
	"net/url"
	"strings"
)

const (
	DriverPG     = "pgx"
	DriverSqlite = "sqlite"
)

type DatabaseConfig struct {
	Url *url.URL `name:"database-url" group:"database" xor:"host,port,user,pwd,db,args" env:"DATABASE_URL" help:"Database connection string. E.g.: postgres://oxidrive:oxidrive@localhost:5432/oxidrive?sslmode=disable or sqlite:///path/to/oxidrive.db"`

	PostgresHost     string `group:"postgresql" xor:"host" required:"" env:"POSTGRES_HOST"`
	PostgresPort     string `group:"postgresql" xor:"port" required:"" env:"POSTGRES_PORT"`
	PostgresUser     string `group:"postgresql" xor:"user" env:"POSTGRES_USER"`
	PostgresPassword string `group:"postgresql" xor:"pwd" env:"POSTGRES_PASSWORD"`
	PostgresDB       string `group:"postgresql" xor:"db" env:"POSTGRES_DB"`
	PostgresArgs     string `group:"postgresql" xor:"args" env:"POSTGRES_ARGS"`
}

func (p *DatabaseConfig) DatabaseUrl() *url.URL {
	if p.Url != nil {
		return p.Url
	}

	creds := ""
	if p.PostgresUser != "" {
		if p.PostgresPassword != "" {
			creds = fmt.Sprintf("%s:%s@", p.PostgresUser, p.PostgresPassword)
		} else {
			creds = fmt.Sprintf("%s@", p.PostgresUser)
		}
	}

	u, err := url.Parse(fmt.Sprintf("postgres://%s%s:%s/%s?%s", creds, p.PostgresHost, p.PostgresPort, p.PostgresDB, p.PostgresArgs))
	if err != nil {
		panic(err)
	}

	p.Url = u

	return u
}

func (p *DatabaseConfig) DatabaseSource() string {
	url := p.DatabaseUrl().String()

	switch p.DatabaseDriver() {
	case DriverPG:
		return url
	case DriverSqlite:
		return strings.Split(url, "://")[1]
	default:
		panic("unreachable")
	}
}

func (p *DatabaseConfig) DatabaseName() string {
	u := p.DatabaseUrl()
	switch u.Scheme {
	case "postgres", "postgresql":
		return "postgres"
	case "sqlite":
		return "sqlite"
	default:
		panic(fmt.Sprintf("unsupported database protocol: %s", u.Scheme))
	}
}

func (p *DatabaseConfig) DatabaseDriver() string {
	switch p.DatabaseName() {
	case "postgres":
		return DriverPG
	case "sqlite":
		return DriverSqlite
	default:
		panic("unreachable")
	}
}
