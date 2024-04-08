package web

import (
    "fmt"
    "net/http"
)

func Run() error {
    port := ":3333"

    router := routes()
    
    fmt.Printf("starting oxidrive server on 0.0.0.0%s", port)
    return http.ListenAndServe(port, router)
}