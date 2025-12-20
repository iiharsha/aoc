package day2

import (
	"fmt"
	"log"
	"strings"

	"github.com/iiharsha/aoc/gopher/internal/aoclib"
)

type Day2 struct {
	ranges []*rangeID
}

func (d *Day2) Name() (int, int) {
	return 2025, 2
}

func (d *Day2) Parse() {
	year, day := d.Name()
	input, err := aoclib.ReadDayLines(year, day)
	if err != nil {
		log.Fatalf("err: %s -> %d", err, year)
	}

	for _, line := range input {
		line = strings.TrimSpace(line)

		for pairs := range strings.SplitSeq(line, ",") {
			if pairs == "" {
				continue
			}
			bounds := strings.Split(pairs, "-")
			if len(bounds) != 2 {
				log.Fatalf("invalid range: %s", pairs)
			}

			start := aoclib.MustUint(bounds[0])
			end := aoclib.MustUint(bounds[1])

			d.ranges = append(d.ranges, &rangeID{
				start: start,
				end:   end,
			})
		}
	}

	for _, rng := range d.ranges {
		fmt.Println("range: ", *rng)
	}
}

func (d *Day2) PartOne() []string {
	return aoclib.Output("unsolved")
}

func (d *Day2) PartTwo() []string {
	return aoclib.Output("unsolved")
}

type rangeID struct {
	start uint
	end   uint
}
