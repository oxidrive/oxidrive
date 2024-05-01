package config

type StorageConfig struct {
	StoragePrefix string `group:"storage" default:"/files"`
}
