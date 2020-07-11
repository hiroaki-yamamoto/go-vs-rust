package main

import "net/http"

type handler struct{}

func (me *handler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte("Hello World"))
}

func main() {
	http.ListenAndServe(":5000", &handler{})
}
