package main

import "fmt"

func part1() {
	player1pos := 0
	player2pos := 9
	player1score := 0
	player2score := 0

	// 100 sided dice
	dice := -1
	rolls := 0

	roll := func() int {
		dice = (dice + 1) % 100
		rolls += 1
		return dice + 1
	}

	for {
		player1pos = (player1pos + roll() + roll() + roll()) % 10
		player1score += player1pos + 1

		if player1score >= 1000 {
			fmt.Println("Player 1 wins", player2score*rolls)
			break
		}

		player2pos = (player2pos + roll() + roll() + roll()) % 10
		player2score += player2pos + 1

		if player2score >= 1000 {
			fmt.Println("Player 2 wins", player1score*rolls)
			break
		}

	}
}

func part2() {
	// quantum dice time
	// this is another frequency game
	// I keep track of the frequency of occurances of play positions and scores and based on the rules of the game those frequencies update.
	// at most there are 2 players, like 30 possibles scores, and 10 possibles positions.
	// 2 * 30 * 10 = 600 possibles even if I'm off by multiple orders It should fit in memory
	type Game struct {
		player1pos   int
		player2pos   int
		player1score int
		player2score int
	}

	// maps the value after 3 dirac rolls to their frequencies, in total there are 27 different futures
	futures := map[int]int{
		3: 1,
		4: 3,
		5: 6,
		6: 7,
		7: 6,
		8: 3,
		9: 1,
	}

	frequencies := make(map[Game]int)
	nextFrequencies := make(map[Game]int)

	// seed the game with the initial state
	game := Game{0, 9, 0, 0}
	frequencies[game] = 1

	// track the number of wins for each player
	player1wins := 0
	player2wins := 0

	// 1 for player 1, 2 for player 2
	turn := 1

	// run until there are no more running games
Simulation:
	for {
		if turn == 1 {
			// player 1's turn
			for game, frequency := range frequencies {
				for positions, future := range futures {
					position := (game.player1pos + positions) % 10
					score := game.player1score + position + 1

					// if the score is atleast 21 stop the game
					if score >= 21 {
						player1wins += frequency * future
					} else {
						nextFrequencies[Game{position, game.player2pos, score, game.player2score}] += frequency * future
					}
				}
			}
			turn = 2
		} else {
			// player 2's turn
			for game, frequency := range frequencies {
				for positions, future := range futures {
					position := (game.player2pos + positions) % 10
					score := game.player2score + position + 1

					// if the score is atleast 21 stop the game
					if score >= 21 {
						player2wins += frequency * future
					} else {
						nextFrequencies[Game{game.player1pos, position, game.player1score, score}] += frequency * future
					}
				}
			}
			turn = 1
		}

		// swap the frequencies
		frequencies, nextFrequencies = nextFrequencies, make(map[Game]int)

		// check if there are any games left
		for _, frequency := range frequencies {
			if frequency != 0 {
				continue Simulation
			}
		}

		break
	}

	fmt.Println()
	fmt.Println("Player 1 wins", player1wins)
	fmt.Println("Player 2 wins", player2wins)
}

func main() {
	part1()
	part2()
}
