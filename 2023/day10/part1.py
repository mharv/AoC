# Description: Advent of Code: Day 10, 2023
# Created by: Mitchell Harvey

example = """..F7.
.FJ|.
SJ.L7
|F--J
LJ..."""


def process_input(input):
    output = []
    for line in input.splitlines():
        temp_line = []
        for char in line:
            temp_line.append(char)
        output.append(temp_line)
    return output


def find_start(matrix):
    for i, line in enumerate(matrix):
        for j, char in enumerate(line):
            if char == "S":
                return (i, j)


def find_next_pipe(matrix, position, previous_position):
    # choose start tile ( this was over engineered)
    if matrix[position[0]][position[1]] == "S":
        if not position[1] - 1 < 0:
            if matrix[position[0]][position[1] - 1] in "L-FS" and (
                position[0] != previous_position[0]
                or position[1] - 1 != previous_position[1]
            ):
                return (position[0], position[1] - 1), position

        if not position[1] + 1 > len(matrix[0]) - 1:
            if matrix[position[0]][position[1] + 1] in "7-JS" and (
                position[0] != previous_position[0]
                or position[1] + 1 != previous_position[1]
            ):
                return (position[0], position[1] + 1), position

        if not position[0] + 1 > len(matrix) - 1:
            if matrix[position[0] + 1][position[1]] in "L|JS" and (
                position[0] + 1 != previous_position[0]
                or position[1] != previous_position[1]
            ):
                return (position[0] + 1, position[1]), position

        if not position[0] - 1 < 0:
            if matrix[position[0] - 1][position[1]] in "7|FS" and (
                position[0] - 1 != previous_position[0]
                or position[1] != previous_position[1]
            ):
                return (position[0] - 1, position[1]), position

    # once you are moving, determine exit based on pipe shape
    else:
        print(matrix[position[0]][position[1]])
        if matrix[position[0]][position[1]] == "J":
            if previous_position[1] != position[1] - 1:
                return (position[0], position[1] - 1), position  # Left
            else:
                return (position[0] - 1, position[1]), position  # Up

        if matrix[position[0]][position[1]] == "F":
            if previous_position[0] != position[0] + 1:
                return (position[0] + 1, position[1]), position  # Down
            else:
                return (position[0], position[1] + 1), position  # Right

        if matrix[position[0]][position[1]] == "L":
            if previous_position[1] != position[1] + 1:
                return (position[0], position[1] + 1), position  # Right
            else:
                return (position[0] - 1, position[1]), position  # Up

        if matrix[position[0]][position[1]] == "7":
            if previous_position[1] != position[1] - 1:
                return (position[0], position[1] - 1), position  # Left
            else:
                return (position[0] + 1, position[1]), position  # Down

        if matrix[position[0]][position[1]] == "-":
            if previous_position[1] != position[1] + 1:
                return (position[0], position[1] + 1), position  # Right
            else:
                return (position[0], position[1] - 1), position  # Left

        if matrix[position[0]][position[1]] == "|":
            if previous_position[0] != position[0] + 1:
                return (position[0] + 1, position[1]), position  # Down
            else:
                return (position[0] - 1, position[1]), position  # Up


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    matrix = process_input(read_input())
    start = find_start(matrix)
    home = start

    print(start)
    count = 0
    previous_position = (len(matrix) - 1, len(matrix[0]) - 1)
    current_position = start

    while True:
        current_position, previous_position = find_next_pipe(
            matrix, current_position, previous_position
        )

        if current_position == start:
            break
        count += 1
    print((count + 1) / 2)


if __name__ == "__main__":
    main()
