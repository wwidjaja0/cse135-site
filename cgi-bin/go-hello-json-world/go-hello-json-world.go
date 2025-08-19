package main

import (
	"encoding/json"
	"net/http"
	"net/http/cgi"
	"time"
)

type Payload struct {
	Title   string `json:"title"`
	Heading string `json:"heading"`
	Author  string `json:"author"`
	Message string `json:"message"`
	Time    string `json:"time"`
	IP      string `json:"ip"`
}

func main() {
	cgi.Serve(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		payload := Payload{
			Title:   "Example JSON Output",
			Heading: "Hello from Go CGI",
			Author:  "William Widjaja",
			Message: "This is a JSON response",
			Time:    time.Now().Format(time.RFC3339),
			IP:      r.RemoteAddr,
		}

		w.Header().Set("Cache-Control", "no-cache")
		w.Header().Set("Content-Type", "application/json")

		if err := json.NewEncoder(w).Encode(payload); err != nil {
			http.Error(w, "Failed to encode JSON", http.StatusInternalServerError)
		}
	}))
}
