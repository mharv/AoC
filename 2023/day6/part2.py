# Description: Advent of Code: Day 6, 2023
# Created by: Mitchell Harvey

example = """Time:      7  15   30
Distance:  9  40  200"""


def process_input(input):
    times = []
    distances = []
    games = []

    times.append(input.splitlines()[0].split(":")[1].replace(" ", ""))
    distances.append(input.splitlines()[1].split(":")[1].replace(" ", ""))

    for t, d in zip(times, distances):
        games.append([int(t), int(d)])

    return games


def find_count_of_winning_input(game):
    count = 0
    for i in range(0, game[0] + 1):
        if i * (game[0] - i) > game[1]:
            count += 1

    return count


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    total = []
    for game in process_input(read_input()):
        print(game)
        total.append(find_count_of_winning_input(game))

    output = 1
    for result in total:
        output *= result
    print(output)


if __name__ == "__main__":
    main()
