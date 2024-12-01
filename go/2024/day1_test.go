package twentyfour_test

import (
	"fmt"
	"io"
	"os"
	"testing"

	twentyfour "src.naesna.es/aoc/2024"
)

// SetupInput reads the input file for the day and returns the contents as a string.
func SetupInput(day string) string {
	f, err := os.Open(fmt.Sprintf("testdata/%s.txt", day))
	if err != nil {
		panic(err)
	}
	defer f.Close()
	
	if b, err := io.ReadAll(f); err != nil {
		panic(err)
	} else {
		return string(b)
	}
}

func TestDay1Part1(t *testing.T) {
	input := SetupInput("ex1")
	got := twentyfour.Day1Part1(input)
	want := 11
	if got != want {
		t.Errorf("Day1Part1() = %d; want %d", got, want)
	}
}

func TestDay1Part2(t *testing.T) {
	input := SetupInput("ex1")
	got := twentyfour.Day1Part2(input)
	want := 31
	if got != want {
		t.Errorf("Day1Part2() = %d; want %d", got, want)
	}
}
