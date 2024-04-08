package main

import (
	"errors"
	"fmt"
	"net/http"
	"os"

	"github.com/oxidrive/oxidrive/internal/web"
)

func main() {
	err := web.Run()

	if errors.Is(err, http.ErrServerClosed) {
		fmt.Println("server closed")
	} else if err != nil {
		fmt.Printf("server stopped: %s\n", err)
		os.Exit(1)
	}
}
