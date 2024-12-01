package util

import "net/http"

type Utils interface {
	GetInputFromFile(string) (string, error)
	GetInputFromOnline(string) (string, error)
}

type AdventUtils struct {
	netClient *http.Client
	Session string
}

type HTTPClient interface {
	Do(*http.Request) (*http.Response, error)
}

func NewAdventUtils(session string) *AdventUtils {
	httpClient := http.DefaultClient

	return &AdventUtils{
		netClient: httpClient,
		Session: session,
	}
}
