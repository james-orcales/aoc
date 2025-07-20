package d2

import (
	"fmt"
	"io"
)

const (
	rock     = 1
	paper    = 2
	scissors = 3
)

var aliases = map[rune]int{
	'A': rock,
	'B': paper,
	'C': scissors,
	'X': rock,
	'Y': paper,
	'Z': scissors,
}

func P1() {
	var (
		opponent rune
		player   rune
	)

	var score int

	for {
		_, err := fmt.Scanf("%c %c\n", &opponent, &player)
		if err == io.EOF {
			break
		}
		score += eval(aliases[opponent], aliases[player]) + aliases[player]
	}
	fmt.Println(score)
}

const (
	win  = 6
	lose = 0
	draw = 3
)

func eval(opponent, player int) (result int) {
	if opponent == player {
		return draw
	} else if (player == rock && opponent == scissors) ||
		(player == paper && opponent == rock) ||
		(player == scissors && opponent == paper) {
		return win
	} else {
		return lose
	}
}

// -----------------------------------------------------
var aliases2 = map[rune]int{
	'A': rock,
	'B': paper,
	'C': scissors,
	'X': lose,
	'Y': draw,
	'Z': win,
}

var beats = map[int]int{
	rock:     scissors,
	paper:    rock,
	scissors: paper,
}

func P2() {
	var (
		opponent rune
		result   rune
	)

	var score int

	for {
		_, err := fmt.Scanf("%c %c\n", &opponent, &result)
		if err == io.EOF {
			break
		}
		score += eval2(aliases2[opponent], aliases2[result]) + aliases2[result]
	}
	fmt.Println(score)
}

func eval2(opponent, result int) (response int) {
	if result == draw {
		response = opponent
	} else if result == win {
		for winner, loser := range beats {
			if loser == opponent {
				response = winner
			}
		}
	} else {
		response = beats[opponent]
	}
	return response
}
