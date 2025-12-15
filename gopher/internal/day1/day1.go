package day1

import (
	"fmt"
	"strconv"

	"github.com/iiharsha/aoc/gopher/internal/aoclib"
)

type Day1 struct {
	turns []*turn
}

func (d *Day1) Name() (int, int) {
	return 2025, 1
}

func (d *Day1) Parse() {
	year, day := d.Name()
	lines := aoclib.ReadDayLines(year, day)
	fmt.Println(lines)

	d.turns = make([]*turn, 0, len(lines))
	for _, line := range lines {
		d.turns = append(d.turns, parseTurn(line))
	}

	for _, t := range d.turns {
		fmt.Println(*t)
	}
}

func (d *Day1) PartOne() []string {
	return aoclib.Output("unsolved")
}

func (d *Day1) PartTwo() []string {
	return aoclib.Output("unsolved")
}

type direction int

const (
	left direction = iota
	right
)

type turn struct {
	dir    direction
	amount int
}

func parseTurn(s string) *turn {
	amount, err := strconv.Atoi(s[1:])
	if err != nil {
		panic("couldn;t convert input string number to integer")
	}

	if s[0] == 'L' {
		return &turn{dir: left, amount: amount}
	}
	return &turn{dir: right, amount: amount}
}
