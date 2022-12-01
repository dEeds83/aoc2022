package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	data, _ := os.ReadFile("../input.txt")
	stringData := string(data)
	//Lines split
	lines := strings.Split(stringData, "\n")
	rainDeers := make([]int, 0)
	calSum := 0
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			rainDeers = append(rainDeers, calSum)
			calSum = 0
			continue
		}
		intVal, _ := strconv.ParseInt(line, 10, 32)
		calSum += int(intVal)
	}

	sort.Ints(rainDeers)

	fmt.Println(fmt.Sprintf("Part A: %d", rainDeers[len(rainDeers)-1]))

	fmt.Println(fmt.Sprintf("Part B: %d", rainDeers[len(rainDeers)-1]+rainDeers[len(rainDeers)-2]+rainDeers[len(rainDeers)-3]))
}
