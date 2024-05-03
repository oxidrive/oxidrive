package api

import "github.com/getkin/kin-openapi/openapi3filter"

//go:generate go run github.com/deepmap/oapi-codegen/v2/cmd/oapi-codegen@master --config=cfg.yaml ../../../openapi/out.yml

func init() {
	openapi3filter.RegisterBodyDecoder("image/jpeg", openapi3filter.FileBodyDecoder)
}
