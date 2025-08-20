package main

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"html"
	"net/http"
	"net/http/cgi"
	"os"
	"path/filepath"
	"strings"
)

const sessionDir = "/tmp"

// Generate a random session ID
func newSessionID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b)
}

// Load or create a session
func getSession(w http.ResponseWriter, r *http.Request) (string, map[string]string) {
	var sid string
	var cookie *http.Cookie
	var err error

	if cookie, err = r.Cookie("CGISESSID"); err == nil {
		sid = cookie.Value
	}

	if sid == "" {
		sid = newSessionID()
		http.SetCookie(w, &http.Cookie{Name: "CGISESSID", Value: sid, Path: "/"})
	}

	sessionFile := filepath.Join(sessionDir, "gosession_"+sid)
	data := make(map[string]string)

	if content, err := os.ReadFile(sessionFile); err == nil {
		for _, line := range strings.Split(string(content), "\n") {
			if kv := strings.SplitN(line, "=", 2); len(kv) == 2 {
				data[kv[0]] = kv[1]
			}
		}
	}

	return sid, data
}

// Save session to file
func saveSession(sid string, data map[string]string) {
	sessionFile := filepath.Join(sessionDir, "gosession_"+sid)
	var lines []string
	for k, v := range data {
		lines = append(lines, k+"="+v)
	}
	_ = os.WriteFile(sessionFile, []byte(strings.Join(lines, "\n")), 0600)
}

func handler(w http.ResponseWriter, r *http.Request) {
	sid, session := getSession(w, r)

	// Get username from session or query param
	name := session["username"]
	if uname := r.FormValue("username"); uname != "" {
		name = uname
		session["username"] = uname
	}
	saveSession(sid, session)

	w.Header().Set("Content-Type", "text/html")
	fmt.Fprint(w, "<html>")
	fmt.Fprint(w, "<head><title>Go Sessions</title></head>")
	fmt.Fprint(w, "<body>")
	fmt.Fprint(w, "<h1>Go Sessions Page</h1>")
	fmt.Fprint(w, `<p style="background-color: yellow;">William Widjaja</p><br/>`)

	if name != "" {
		fmt.Fprintf(w, "<p><b>Name:</b> %s</p>", html.EscapeString(name))
	} else {
		fmt.Fprint(w, "<p><b>Name:</b> You do not have a name set</p>")
	}

	fmt.Fprint(w, "<br/><br/>")
	fmt.Fprint(w, `<a href="/hw2/perl-cgiform.html">Go CGI Form</a><br/>`)
	fmt.Fprint(w, `<form style="margin-top:30px" action="/cgi-bin/go-destroy-session.cgi" method="get">`)
	fmt.Fprint(w, `<button type="submit">Destroy Session</button>`)
	fmt.Fprint(w, "</form>")
	fmt.Fprint(w, "</body></html>")
}

func main() {
	cgi.Serve(http.HandlerFunc(handler))
}
