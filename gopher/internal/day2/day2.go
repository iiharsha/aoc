package day2

import (
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

}

func (d *Day2) PartOne() []string {
	var total uint64
	for _, rng := range d.ranges {
		total += rng.sumInvalidIds()
	}
	return aoclib.Output(total)
}

func (d *Day2) PartTwo() []string {
	return aoclib.Output("unsolved")
}

type rangeID struct {
	start uint64
	end   uint64
}

func (r *rangeID) sumInvalidIds() uint64 {
	var invalidSum uint64
	for num := r.start; num <= r.end; num++ {
		half := (digits(num) + 1) / 2

		for digitCount := uint64(1); digitCount <= half; digitCount++ {
			mod := pow10(digitCount)
			last := num % mod

			if last == 0 || digits(last) != digitCount {
				continue
			}

			test := num / mod
			found := true
			for test > 0 {
				if test % mod != last {
					found = false
					break
				}
				test /= mod
			}

			if found {
				invalidSum += num
				break
			}
		}
	}
	return invalidSum
}

// digits() returns the number of digits
func digits(num uint64) uint64 {
	if num == 0 {
			return 1
	}

	var d uint64
	for num > 0 {
		d++
		num /= 10
	}
	return d
}

func pow10(exp uint64) uint64 {
	var r uint64 = 1
	for i := uint64(0); i < exp; i++ {
		r *= 10
	}
	return r
}
