package main

import (
	"fmt"
	"net/http"
	"net/http/cgi"
	"os"
	"path/filepath"
)

const sessionDir = "/tmp"

func handler(w http.ResponseWriter, r *http.Request) {
	// Get session ID from cookie or param
	var sid string
	if cookie, err := r.Cookie("SITE_SID"); err == nil {
		sid = cookie.Value
	} else {
		sid = r.FormValue("sid")
	}

	// Delete session file if it exists
	if sid != "" {
		sessionFile := filepath.Join(sessionDir, "gosession_"+sid)
		_ = os.Remove(sessionFile)

		// Expire cookie
		http.SetCookie(w, &http.Cookie{
			Name:   "SITE_SID",
			Value:  "",
			Path:   "/",
			MaxAge: -1,
		})
	}

	// Print HTML response
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprint(w, "<html>")
	fmt.Fprint(w, "<head><title>Go Session Destroyed</title></head>")
	fmt.Fprint(w, "<body>")
	fmt.Fprint(w, "<h1>Session Destroyed</h1>")
	fmt.Fprint(w, `<p style="background-color: yellow;">William Widjaja</p><br/>`)
	fmt.Fprint(w, `<a href="/hw2/go-cgiform.html">Back to the Go CGI Form</a><br/>`)
	fmt.Fprint(w, `<a href="/cgi-bin/go-session.cgi">Back to Page 1</a><br/>`)
	fmt.Fprint(w, "</body></html>")
}

func main() {
	cgi.Serve(http.HandlerFunc(handler))
}
