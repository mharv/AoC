# Description: Advent of Code: Day 3, 2023
# Created by: Mitchell Harvey

example = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""


def create_overlaps(input):
    overlaps = [list(line) for line in input]
    width = len(input[0]) - 1
    height = len(input) - 1

    for i, line in enumerate(input):
        for j, char in enumerate(line):
            if not char.isnumeric() and char != ".":
                if j > 0 and j < width:
                    overlaps[i][j - 1] = char
                    overlaps[i][j + 1] = char
                if i > 0 and i < height:
                    overlaps[i - 1][j] = char
                    overlaps[i + 1][j] = char
                if i > 0 and i < height and j > 0 and j < width:
                    overlaps[i - 1][j - 1] = char
                    overlaps[i - 1][j + 1] = char
                    overlaps[i + 1][j - 1] = char
                    overlaps[i + 1][j + 1] = char

    return overlaps


def return_overlaps(input, overlaps):
    flatten_input = [j for sub in input for j in sub]
    flatten_overlaps = [j for sub in overlaps for j in sub]

    overlapping_numbers = []
    temp_number = ""
    overlapped = False
    for i, j in zip(flatten_input, flatten_overlaps):
        if i.isnumeric():
            if not j.isnumeric() and j != ".":
                overlapped = True
            temp_number += i

        if not i.isnumeric() and len(temp_number) > 0:
            if overlapped:
                overlapping_numbers.append(int(temp_number))
            temp_number = ""
            overlapped = False
    return overlapping_numbers


def convert_to_matrix(input):
    output = []
    for i, line in enumerate(input.splitlines()):
        output.append([])
        for j, char in enumerate(line):
            output[i].append(char)
    return output


def print_grid(grid):
    for line in grid:
        print(line)


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input_converted = convert_to_matrix(read_input())
    total = 0
    overlaps = create_overlaps(input_converted)
    print_grid(input_converted)
    print("\n")
    print_grid(overlaps)
    print("\n")
    print(sum(return_overlaps(input_converted, overlaps)))


if __name__ == "__main__":
    main()
