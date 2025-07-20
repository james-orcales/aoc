package d3

import (
	"bufio"
	"os"
	"strings"
)

// vJrwpWtwJgWrhcsFMMfFFhFp

func P1() {
	reader := bufio.NewScanner(os.Stdin)
	var sum int
	for reader.Scan() {
		l := len(reader.Text())
		sackcmp1 := reader.Text()[:l/2]
		sackcmp2 := reader.Text()[l-l/2:]
		for _, r := range sackcmp1 {
			if strings.ContainsRune(sackcmp2, r) {
				sum += priority(r)
				break
			}
		}
	}
	println(sum)
}

func P2() {
	reader := bufio.NewScanner(os.Stdin)

    var x, y string
    for reader.Scan(){
        if x == "" {
            x = reader.Text()
            continue
        } else {
            y = reader.Text()
        }
    }
}

func priority(item rune) int {
	if 'a' <= item && item <= 'z' {
		return int(item - 'a' + 1)
	} else if 'A' <= item && item <= 'Z' {
		return int(item - 'A' + 27)
	} else {
		return -1
	}
}
