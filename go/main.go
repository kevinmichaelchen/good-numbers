package main

import (
	"fmt"
	"time"
)

const UpperBound = 1e9

func isGoodNumber(x int) bool {
	sum := 0
	tmp := x
	for tmp > 0 {
		n := tmp
		tmp = tmp / 10;
		sum += n - tmp * 10;
	}
	return x % sum == 0
}

func main() {
	start := time.Now()
	goodNumbers := []int{}
	for i := 1; i <= UpperBound; i++ {
		if isGoodNumber(i) {
			goodNumbers = append(goodNumbers, i)
		}
	}
	done := time.Now()
	fmt.Printf("There are %d good numbers\n", len(goodNumbers))
	fmt.Printf("Program duration: %v\n", done.Sub(start))
}
