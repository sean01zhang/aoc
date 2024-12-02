package util

import (
	"fmt"
	"io"
	"net/http"
	"os"
)

// HTTPClient is an interface that defines the Do method.
type HTTPClient interface {
	Do(*http.Request) (*http.Response, error)
}

// GetInputFromOnline fetches the text from the given filepath.
func GetInputFromFile(path string) (string, error) {
	f, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer f.Close()

	if b, err := io.ReadAll(f); err != nil {
		return "", err
	} else {
		return string(b), nil
	}
}

type onlineInputOpts struct {
	Session   string
	netClient HTTPClient
}

type OnlineInputOpt func(*onlineInputOpts)

func WithSession(session string) OnlineInputOpt {
	return func(o *onlineInputOpts) {
		o.Session = session
	}
}

// GetInputFromOnline fetches the body text from the given URL.
func GetInputFromOnline(url string, opts ...OnlineInputOpt) (string, error) {
	inputOpts := onlineInputOpts{
		netClient: http.DefaultClient,
	}
	for _, o := range opts {
		o(&inputOpts)
	}

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("Cookie", fmt.Sprintf("session=%s", inputOpts.Session))

	resp, err := inputOpts.netClient.Do(req)
	if err != nil {
		return "", err
	}

	defer resp.Body.Close()
	if b, err := io.ReadAll(resp.Body); err != nil {
		return "", err
	} else {
		return string(b), nil
	}
}
