package main

import (
	"fmt"
	"html/template"
	"io"
	"net/http"
	"net/http/cgi"
	"net/url"
	"os"
	"strconv"
	"strings"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-Type", "text/html")

		fmt.Fprint(w, "<html><head><title>POST Message Body</title></head>")
		fmt.Fprint(w, `<body><h1 align="center">Post Message Body</h1><hr>`)
		fmt.Fprint(w, `<p style="background-color: yellow;">William Widjaja</p><br />`)

		contentLengthStr := os.Getenv("CONTENT_LENGTH")
		contentLength, err := strconv.Atoi(contentLengthStr)
		if err != nil || contentLength <= 0 {
			contentLength = 0
		}

		var body string
		if contentLength > 0 {
			buf := make([]byte, contentLength)
			if _, err := io.ReadFull(os.Stdin, buf); err == nil {
				body = string(buf)
			}
		}

		if strings.TrimSpace(body) == "" {
			fmt.Fprint(w, "<b>Message Body:</b> (null)<br />\n")
		} else {
			fmt.Fprintf(w, "<b>Message Body:</b> %s<br />\n", template.HTMLEscapeString(body))
		}

		params, err := url.ParseQuery(body)
		if err != nil || len(params) == 0 {
			fmt.Fprint(w, "<p><em>No form parameters were provided.</em></p>")
		} else {
			fmt.Fprint(w, "<ul>")
			for key, vals := range params {
				for _, val := range vals {
					fmt.Fprintf(w, "<li><strong>%s</strong>: %s</li>\n",
						template.HTMLEscapeString(key),
						template.HTMLEscapeString(val))
				}
			}
			fmt.Fprint(w, "</ul>")
		}

		fmt.Fprint(w, "</body></html>")
	}))
}
