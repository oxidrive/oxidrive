package api

import (
	"github.com/getkin/kin-openapi/openapi3filter"
	"gitlab.com/gitlab-org/go-mimedb"
)

//go:generate go run github.com/deepmap/oapi-codegen/v2/cmd/oapi-codegen@master --config=cfg.yaml ../../../openapi/out.yml

func init() {
	for ct := range mimedb.MimeTypeToExts {
		if r := openapi3filter.RegisteredBodyDecoder(ct); r == nil {
			openapi3filter.RegisterBodyDecoder(ct, openapi3filter.FileBodyDecoder)
		}
	}
}
