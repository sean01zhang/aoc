package twentyfour_test

import (
	twentyfour "src.naesna.es/aoc/go/2024"
	"testing"
)

func TestDay4Part1(t *testing.T) {
	input := SetupInput("ex4")

	got, err := twentyfour.Day4Part1(input)
	if err != nil {
		t.Errorf("Day4Part1() error = %v; want nil", err)
	}
	if want := "18"; got != want {
		t.Errorf("Day4Part1() = %s; want %s", got, want)
	}
}

func TestDay4Part2(t *testing.T) {
	input := SetupInput("ex4")

	got, err := twentyfour.Day4Part2(input)
	if err != nil {
		t.Errorf("Day4Part2() error = %v; want nil", err)
	}
	if want := "9"; got != want {
		t.Errorf("Day4Part2() = %s; want %s", got, want)
	}
}
