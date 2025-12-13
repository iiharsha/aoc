package day1

import (
	"fmt"

	"github.com/iiharsha/aoc/gopher/internal/aoclib"
)

type Day1 struct {
	// do something here
}

func (d *Day1) Name() (int, int) {
	return 2025, 1
}

func (d *Day1) Parse() {
	lines := aoclib.ReadLines("input/day_1_test.txt")
	fmt.Println(lines)
}

func (d *Day1) PartOne() []string {
	return aoclib.Output("unsolved")
}

func (d *Day1) PartTwo() []string {
	return aoclib.Output("unsolved")
}
