package util

import (
	"fmt"
	"io"
	"net/http"
	"os"
)

func (a AdventUtils) GetInputFromFile(path string) (string, error) {
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

func (a AdventUtils) GetInputFromOnline(url string) (string, error) {
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("Cookie", fmt.Sprintf("session=%s", a.Session))

	resp, err := a.netClient.Do(req)
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
