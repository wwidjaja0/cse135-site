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
			Title:   "Hello, Go!",
			Heading: "Hello, Go!",
			Author:  "William Widjaja",
			Message: "This page was generated with the Go programming language",
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
