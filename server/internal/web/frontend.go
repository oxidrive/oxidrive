package web

import (
	"net/http"
)

func serveFrontend(folderPath string) http.HandlerFunc {
	files := http.FileServer(http.Dir(folderPath))
	return func(w http.ResponseWriter, r *http.Request) {
		files.ServeHTTP(w, r)
	}
}
