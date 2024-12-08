package twentyfour_test

import (
	"fmt"
	"testing"

	twentyfour "src.naesna.es/aoc/go/2024"
	"src.naesna.es/aoc/go/internal/util"
)

// SetupInput reads the input file for the day and returns the contents as a string.
func SetupInput(day string) string {
	input, err := util.GetInputFromFile(fmt.Sprintf("testdata/%s.txt", day))
	if err != nil {
		panic(err)
	}

	return input
}

func TestDay1Part1(t *testing.T) {
	input := SetupInput("ex1")
	got, err := twentyfour.Day1Part1(input)
	if err != nil {
		t.Errorf("Day1Part1() error = %v; want nil", err)
	}
	if want := "11"; got != want {
		t.Errorf("Day1Part1() = %s; want %s", got, want)
	}
}

func TestDay1Part2(t *testing.T) {
	input := SetupInput("ex1")
	got, err := twentyfour.Day1Part2(input)
	if err != nil {
		t.Errorf("Day1Part2() error = %v; want nil", err)
	}
	if want := "31"; got != want {
		t.Errorf("Day1Part2() = %s; want %s", got, want)
	}
}

func BenchmarkDay1Part1(b *testing.B) {
	input := SetupInput("day1")
	b.ResetTimer()

	for n := 0; n < b.N; n++ {
		twentyfour.Day1Part1(input)
	}
}

