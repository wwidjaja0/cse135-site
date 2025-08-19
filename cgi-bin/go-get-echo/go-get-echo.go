package main

import (
	"fmt"
	"html/template"
	"net/http"
	"net/http/cgi"
	"net/url"
)

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-Type", "text/html")

		fmt.Fprint(w, "<html><head><title>Get Request Echo</title></head><body><h1 align=\"center\">Get Request Echo</h1><hr>")
		fmt.Fprint(w, "<p style=\"background-color: yellow;\">William Widjaja</p><br />")

		query := r.URL.RawQuery
		fmt.Fprintf(w, "<b>Query String:</b> %s<br />\n", template.HTMLEscapeString(query))

		values, err := url.ParseQuery(query)
		if err != nil || len(values) == 0 {
			fmt.Fprint(w, "<p><em>No query parameters were provided.</em></p>")
		} else {
			fmt.Fprint(w, "<ul>")
			for key, vals := range values {
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
