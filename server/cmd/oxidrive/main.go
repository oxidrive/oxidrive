package main

import (
    "fmt"
    "os"
    "github.com/oxidrive/oxidrive/internal/web"
)

func main() {
    if err := web.Run(); err != nil {
        fmt.Printf("server stopped: %s\n", err)
        os.Exit(1)
    }
}