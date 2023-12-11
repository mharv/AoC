# Description: Advent of Code: Day 11, 2023
# Created by: Mitchell Harvey

example = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""


def process_input(input):
    return input


def expand(input):
    rows_to_expand = []
    cols_to_expand = []
    for i, line in enumerate(input.splitlines()):
        if all(char == "." for char in line):
            rows_to_expand.append(i)
    # loop through columns
    for i in range(len(input.splitlines()[0])):
        column = ""
        for line in input.splitlines():
            column += line[i]
        if all(char == "." for char in column):
            cols_to_expand.append(i)
    return rows_to_expand, cols_to_expand


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input = read_input()
    for line in input.splitlines():
        print(line)
    print(expand(input))


if __name__ == "__main__":
    main()
