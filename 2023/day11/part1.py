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

    for i in range(len(input.splitlines()[0])):
        column = ""
        for line in input.splitlines():
            column += line[i]
        if all(char == "." for char in column):
            cols_to_expand.append(i)

    row_count = 0
    for i in rows_to_expand:
        input = input.splitlines()
        input.insert(i + row_count, "." * len(input[0]))
        input = "\n".join(input)
        row_count += 1

    col_count = 0
    for i in cols_to_expand:
        input = input.splitlines()
        for j, line in enumerate(input):
            input[j] = line[: i + col_count] + "." + line[i + col_count :]
        input = "\n".join(input)
        col_count += 1
    return input


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input = read_input()
    input = expand(input)
    for line in input.splitlines():
        print(line)


if __name__ == "__main__":
    main()
