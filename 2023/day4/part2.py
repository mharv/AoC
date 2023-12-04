# Description: Advent of Code: Day 4, 2023
# Created by: Mitchell Harvey

example = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""


class Game:
    def __init__(self, game_no, winning_numbers=[], my_numbers=[], instances=1):
        self.game_no = game_no
        self.winning_numbers = []
        self.my_numbers = []
        self.instances = 1


def process_cards(input):
    games = []

    for line in input.splitlines():
        temp_game = Game(line.split(":")[0].split(" ")[1])

        string_processed = line.split(":")[1].strip().split("|")
        winning_string = string_processed[0].strip()
        my_string = string_processed[1].strip()

        for number in winning_string.split():
            temp_game.winning_numbers.append(number)

        for number in my_string.split():
            temp_game.my_numbers.append(number)

        games.append(temp_game)

    return games


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    games = process_cards(read_input())
    for i, game in enumerate(games):
        temp_total = 0
        matches = 0
        for number in game.my_numbers:
            if number in game.winning_numbers and matches > 0:
                matches += 1
                temp_total *= 2
            if number in game.winning_numbers and matches == 0:
                matches += 1
                temp_total += 1
        print(f"{game.game_no} has {matches} matches")
        for match in range(1, matches + 1):
            print("game_no")
            print(games[match + i].game_no)
            games[match + i].instances += 1 * game.instances
        # total += temp_total * game.instances

    total = 0
    for game in games:
        total += game.instances
    print(total)


if __name__ == "__main__":
    main()
