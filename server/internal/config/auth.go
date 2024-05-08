package config

import "time"

type AuthConfig struct {
	SessionDuration time.Duration `group:"auth" default:"5m" env:"OXIDRIVE_SESSION_DURATION"`
}
