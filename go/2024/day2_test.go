package twentyfour_test

import (
	"testing"

	twentyfour "src.naesna.es/aoc/go/2024"
)

func TestDay2Part1(t *testing.T) {
	input := SetupInput("ex2")

	got, err := twentyfour.Day2Part1(input)
	if err != nil {
		t.Errorf("Day2Part1() error = %v; want nil", err)
	}
	if want := "2"; got != want {
		t.Errorf("Day2Part1() = %s; want %s", got, want)
	}
}

func TestDay2Part2(t *testing.T) {
	input := SetupInput("ex2")

	got, err := twentyfour.Day2Part2(input)
	if err != nil {
		t.Errorf("Day2Part2() error = %v; want nil", err)
	}
	if want := "4"; got != want {
		t.Errorf("Day2Part2() = %s; want %s", got, want)
	}
}
