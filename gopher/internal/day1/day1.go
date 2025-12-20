package day1

import (
	"log"
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
	lines, err := aoclib.ReadDayLines(year, day)
	if err != nil {
		log.Fatalf("err: %s -> %d", err, year)
	}

	d.turns = make([]*turn, 0, len(lines))
	for _, line := range lines {
		d.turns = append(d.turns, parseTurn(line))
	}
}

const startPos = 50

func (d *Day1) PartOne() []string {
	setting := startPos
	counter := 0
	for _, t := range d.turns {
		if t.dir == left {
			setting = remEuclid(setting-t.amount, 100)
		} else {
			setting = (setting + t.amount) % 100
		}

		if setting == 0 {
			counter++
		}
	}
	return aoclib.Output(counter)
}

func (d *Day1) PartTwo() []string {
	setting := startPos
	counter := 0
	for _, t := range d.turns {
		var dir = 1
		if t.dir == left {
			dir = -1
		}

		for t.amount > 0 {
			setting = remEuclid(setting+dir, 100)
			if setting == 0 {
				counter++
			}

			t.amount--
		}
	}

	return aoclib.Output(counter)
}

func remEuclid(a, b int) int {
	r := a % b
	if r < 0 {
		r += b
	}
	return r
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
		panic("error: converting input string number to integer")
	}

	if s[0] == 'L' {
		return &turn{dir: left, amount: amount}
	}
	return &turn{dir: right, amount: amount}
}
