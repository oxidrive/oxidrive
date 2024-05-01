package config

type StorageConfig struct {
	StoragePrefix      string `group:"storage" default:"/files"`
	ThroughputInByte   int    `group:"storage" default:"32" env:"STORAGE_COPY_THROUGHPUT" help:"It can be use to control the validation frequency while a file is stored permanently."`
	MultipartMaxMemory int    `group:"storage" default:"32" env:"STORAGE_MULTIPART_MAX_MEMORY" help:"It can be use to limit the memory footprint at speed costs."`
}
