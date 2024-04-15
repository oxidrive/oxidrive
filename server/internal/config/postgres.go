package config

import "fmt"

type PostgresConfig struct {
	DatabaseUrl string `group:"postgresql" xor:"host,port,user,pwd,db,args" env:"DATABASE_URL"`

	PostgresHost     string `group:"postgresql" xor:"host" required:"" env:"POSTGRES_HOST"`
	PostgresPort     string `group:"postgresql" xor:"port" required:"" env:"POSTGRES_PORT"`
	PostgresUser     string `group:"postgresql" xor:"user" env:"POSTGRES_USER"`
	PostgresPassword string `group:"postgresql" xor:"pwd" env:"POSTGRES_PASSWORD"`
	PostgresDB       string `group:"postgresql" xor:"db" env:"POSTGRES_DB"`
	PostgresArgs     string `group:"postgresql" xor:"args" env:"POSTGRES_ARGS"`
}

func (p *PostgresConfig) Url() string {
	if p.DatabaseUrl != "" {
		return p.DatabaseUrl
	}

	creds := ""
	if p.PostgresUser != "" {
		if p.PostgresPassword != "" {
			creds = fmt.Sprintf("%s:%s@", p.PostgresUser, p.PostgresPassword)
		} else {
			creds = fmt.Sprintf("%s@", p.PostgresUser)
		}
	}

	return fmt.Sprintf("postgres://%s%s:%s/%s?%s", creds, p.PostgresHost, p.PostgresPort, p.PostgresDB, p.PostgresArgs)

}
