package main

import (
	"fmt"
	"net/http"
	"net/http/cgi"
	"time"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-type", "text/html")

		now := time.Now().Format(time.RFC3339)
		ip := r.RemoteAddr

		fmt.Fprint(w, "<html><body>")
		fmt.Fprint(w, "<h1>Hello, Go!</h1>")
		fmt.Fprint(w, "<p style=\"background-color: yellow;\">William Widjaja</p>")
		fmt.Fprint(w, "<br />")
		fmt.Fprint(w, "<p>This page was generated with the Go programming langauge</p>")
		fmt.Fprintf(w, "Current Time: %s", now)
		fmt.Fprint(w, "<br />")
		fmt.Fprintf(w, "Your IP Address: %s", ip)
		fmt.Fprint(w, "</body></html>")
	}))
}
