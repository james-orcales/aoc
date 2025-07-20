package d1

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func P2() {
    reader := bufio.NewScanner(os.Stdin)
    var elves []int
    var elfCalories int

    for reader.Scan() {
        caloriesText := reader.Text()
        if caloriesText == ""{
            elves = append(elves, elfCalories)
            elfCalories = 0
            continue
        }

        calories, err := strconv.Atoi(caloriesText)
        if err != nil {
            fmt.Println(err)
            return
        }
        elfCalories += calories
    }
    if reader.Err() == nil {
        elves = append(elves, elfCalories)
    }

    sort.Ints(elves)
    var totalCalories int
    for _, x := range elves[len(elves)-3:]{
        totalCalories += x
    }
    fmt.Println("Total calories:", totalCalories)
}

