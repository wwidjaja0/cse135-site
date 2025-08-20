package main

import (
	"fmt"
	"html/template"
	"io"
	"net/http"
	"net/http/cgi"
	"os"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-Type", "text/html")

		fmt.Fprint(w, `<html><head><title>General Request Echo</title></head>
		<body><h1 align="center">General Request Echo</h1><hr/>`)

		fmt.Fprint(w, `<p style="background-color: yellow;">William Widjaja</p><br />`)
		fmt.Fprint(w, "<table>")

		protocol := os.Getenv("SERVER_PROTOCOL")
		if protocol == "" {
			protocol = "(unknown)"
		}
		method := os.Getenv("REQUEST_METHOD")
		if method == "" {
			method = "(unknown)"
		}

		fmt.Fprintf(w, "<tr><td>Protocol:</td><td>%s</td></tr>\n", template.HTMLEscapeString(protocol))
		fmt.Fprintf(w, "<tr><td>Method:</td><td>%s</td></tr>\n", template.HTMLEscapeString(method))

		// Read up to 1000 bytes from stdin
		buf := make([]byte, 1000)
		n, err := io.ReadFull(io.LimitReader(os.Stdin, 1000), buf)
		var messageBody string
		if err == nil && n > 0 {
			messageBody = string(buf[:n])
		} else {
			messageBody = "(null)"
		}

		fmt.Fprintf(w, "<tr><td>Message Body:</td><td>%s</td></tr>\n", template.HTMLEscapeString(messageBody))

		fmt.Fprint(w, "</table></body></html>")
	}))
}
