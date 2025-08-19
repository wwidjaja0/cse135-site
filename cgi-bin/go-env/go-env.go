package main

import (
	"fmt"
	"net/http"
	"net/http/cgi"
	"os"
	"strings"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-type", "text/html")

		fmt.Fprint(w, "<html><head><title>Environment Variables</title></head><body><h1 align=\"center\">Environment Variables</h1><hr>")

		fmt.Fprint(w, "<p style=\"background-color: yellow;\">William Widjaja</p>")
		fmt.Fprint(w, "<br />")

		for _, env := range os.Environ() {
			parts := strings.SplitN(env, "=", 2)
			key := parts[0]
			value := ""
			if len(parts) > 1 {
				value = parts[1]
			}
			fmt.Fprintf(w, "%s: %s<br />\n", key, value)
		}

		fmt.Fprint(w, "</body></html>")
	}))
}
