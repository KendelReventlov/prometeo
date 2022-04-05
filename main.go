package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
    var nombre_actual string = ""
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        nombre := r.URL.Query().Get("nombre")
        if nombre != ""{
            nombre_actual = nombre
        }
        if nombre_actual == ""{
            nombre_actual = "nada"
        }
        fmt.Fprintf(w, nombre_actual)
    })

    http.ListenAndServe(":"+os.Getenv("PORT"), nil)
}