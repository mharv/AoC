# Description: Advent of Code: Day 2, 2022
# Created by: Mitchell Harvey

test_case = """A Y
B X
C Z"""


mapping = {
    "A": "Rock",
    "B": "Paper",
    "C": "Scissors",
    "X": "Rock",
    "Y": "Paper",
    "Z": "Scissors",
}


extra_score_mapping = {
    "Rock": 1,
    "Paper": 2,
    "Scissors": 3,
}


def read_input():
    with open("./input.txt") as file:
        return file.read()


def process_input(input):
    opponent_moves = []
    my_moves = []
    for line in input.splitlines():
        opponent_moves.append(line.split(" ")[0])
        my_moves.append(line.split(" ")[1])
    return opponent_moves, my_moves


def add_extra_score(move):
    return extra_score_mapping[move]


def decide_score(opp_move, my_move):
    result = 0
    if mapping[opp_move] == mapping[my_move]:
        result = 3 + add_extra_score(mapping[my_move])
    if mapping[opp_move] == "Rock":
        if mapping[my_move] == "Scissors":
            result = 0 + add_extra_score(mapping[my_move])
        if mapping[my_move] == "Paper":
            result = 6 + add_extra_score(mapping[my_move])
    if mapping[opp_move] == "Paper":
        if mapping[my_move] == "Rock":
            result = 0 + add_extra_score(mapping[my_move])
        if mapping[my_move] == "Scissors":
            result = 6 + add_extra_score(mapping[my_move])
    if mapping[opp_move] == "Scissors":
        if mapping[my_move] == "Paper":
            result = 0 + add_extra_score(mapping[my_move])
        if mapping[my_move] == "Rock":
            result = 6 + add_extra_score(mapping[my_move])
    return result


def score_turns(opponent_moves, my_moves):
    total_score = 0
    for opponent_move, my_move in zip(opponent_moves, my_moves):
        total_score = total_score + decide_score(opponent_move, my_move)
    return total_score


def main():
    # opponent_moves, my_moves = process_input(test_case)
    opponent_moves, my_moves = process_input(read_input())
    print(score_turns(opponent_moves, my_moves))


if __name__ == "__main__":
    main()
