package main

import (
	"fmt"
	"net/http"
	"net/http/cgi"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		fmt.Fprint(w, "<html><body>")
		fmt.Fprint(w, "<h1>Hello from Go CGI!</h1>")
		if err := r.ParseForm(); err == nil {
			fmt.Fprintf(w, "<p>Received data: %v</p>", r.Form)
		}
		fmt.Fprint(w, "</body></html>")
	}))
}
