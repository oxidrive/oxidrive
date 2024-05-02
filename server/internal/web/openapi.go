package web

import (
	"fmt"
	"net/http"

	"github.com/swaggest/swgui/v5emb"

	"github.com/oxidrive/oxidrive/server/openapi"
)

func mountSwagger(router *http.ServeMux) {
	router.Handle("/api/docs/", v5emb.New(
		"Oxidrive API",
		"/api/openapi.yaml",
		"/api/docs/",
	))

	router.Handle("/api/openapi.yaml", http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Add("content-type", "application/yaml")
		if _, err := w.Write(openapi.Schema); err != nil {
			panic(fmt.Errorf("failed to write OpenAPI schema to response: %w", err))
		}
	}))
}
