# Description: Advent of Code: Day 10, 2023
# Created by: Mitchell Harvey

# example = """FF7FSF7F7F7F7F7F---7
# L|LJ||||||||||||F--J
# FL-7LJLJ||||||LJL-77
# F--JF--7||LJLJ7F7FJ-
# L---JF-JLJ.||-FJLJJ7
# |F|F-JF---7F7-L7L|7|
# |FFJF7L7F-JF7|JL---7
# 7-L-JL7||F7|L7F-7F7|
# L.L7LFJ|||||FJL7||LJ
# L7JLJL-JLJLJL--JLJ.L"""

example = """...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."""


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
    # choose start tile ( this was over engineered) direction that is
    # chosen dependant on order of execution below
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

    # initialize a random cell on the matrix, probably should not be
    # on matrix in case it happens to be near S by some small chance
    previous_position = (len(matrix) - 1, len(matrix[0]) - 1)
    current_position = start

    tunnel = []
    while True:
        current_position, previous_position = find_next_pipe(
            matrix, current_position, previous_position
        )
        tunnel.append(current_position)

        if current_position == start:
            break

    # create a simple overlay matrix "marker" that can be used to denote path (as a cell == 1)
    marker = []
    for i in matrix:
        marker.append([0] * len(i))
    for item in tunnel:
        marker[item[0]][item[1]] = 1

    contained = []
    for i, row in enumerate(marker):
        toggle = False
        for j, char in enumerate(row):
            # if i hit the path, make sure i am crossing it and not following it.
            if (
                char == 1 and matrix[i][j] in "JL|S"
            ):  # S must be included because it is considered vertical in my input
                toggle = not toggle
                # crossed path

            if toggle and char == 0:
                # any empty cell that is found once the path is crossed must be internal
                contained.append((i, j))

    print(len(contained))


if __name__ == "__main__":
    main()
