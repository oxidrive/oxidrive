package config

type StorageConfig struct {
	StoragePrefix      string `group:"storage" default:"/files" env:"OXIDRIVE_STORAGE_PREFIX"`
	ThroughputInByte   int    `group:"storage" default:"32" env:"OXIDRIVE_STORAGE_COPY_THROUGHPUT" help:"It can be use to control the validation frequency while a file is stored permanently."`
	MultipartMaxMemory int64  `group:"storage" default:"32" env:"OXIDRIVE_STORAGE_MULTIPART_MAX_MEMORY" help:"It can be use to limit the memory footprint at speed costs."`

	StorageFSConfig
}

type StorageFSConfig struct {
	StorageFSDataDir string `group:"storage" type:"path" env:"OXIDRIVE_STORAGE_FS_DATA_DIR"`
}
