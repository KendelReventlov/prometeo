package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, r.URL.Query().Get("nombre"))
    })

    http.ListenAndServe(":"+os.Getenv("PORT"), nil)
}