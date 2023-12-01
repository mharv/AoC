# Description: Advent of Code: Day 1, 2023
# Created by: Mitchell Harvey


def read_input():
    with open("./input.txt") as file:
        return file.read()


def main():
    total = 0
    for line in read_input().splitlines():
        stripped_line = list(filter(lambda x: x.isnumeric(), list(line)))
        total += int(stripped_line[0] + stripped_line[-1])
    print(total)


if __name__ == "__main__":
    main()
