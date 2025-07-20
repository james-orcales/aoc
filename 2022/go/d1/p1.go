package d1

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

// Answer for my input was 69693... nice

func P1() { 
    var maxCal int
    var cal int

    reader := bufio.NewReader(os.Stdin)
    for {
        input, err := reader.ReadString('\n')
        if err != nil {
            // Evaluate maxCalories on EOF if applicable
            if err == io.EOF && cal != 0 && cal > maxCal {
                    maxCal = cal
            }
            break
        }

        input = strings.TrimSuffix(input, "\n")

        if input == "" {
            if cal > maxCal {
                maxCal = cal
            }
            cal = 0
            continue
        }

        input_int, err := strconv.Atoi(input)
        if err != nil {
            fmt.Println(err)
            break
        }
        cal += input_int
    }
    fmt.Println("Most:", maxCal)
}

