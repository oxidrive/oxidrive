package config

type StorageConfig struct {
	StorageRoot string `group:"storage" default:"/oxidrive/storage/files"`
}
