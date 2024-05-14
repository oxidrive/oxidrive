package config

import "time"

type AuthConfig struct {
	SessionDuration time.Duration `group:"auth" default:"24h" env:"OXIDRIVE_SESSION_DURATION"`
}
