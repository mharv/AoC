# Description: Advent of Code: Day 14, 2023
# Created by: Mitchell Harvey

import functools

example = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""


print_matrix = lambda matrix: print("\n".join([" ".join(line) for line in matrix]))


def process_input(input):
    output = []
    for line in input.splitlines():
        output.append(list(line))
    return output


def rotate_matrix_clockwise(matrix):
    try:
        return [
            [matrix[j][i] for j in range(len(matrix) - 1, -1, -1)]
            for i in range(len(matrix[0]))
        ]
    except:
        return IndexError


def rotate_matrix_counterclockwise(matrix):
    try:
        return [
            [matrix[i][j] for i in range(len(matrix))]
            for j in range(len(matrix[0]) - 1, -1, -1)
        ]
    except IndexError:
        return IndexError


def apply_lever(matrix):
    swaps_made = 0
    for row_index, row in enumerate(matrix):
        for i, item in enumerate(row):
            if i + 1 == len(row):
                break
            if item == "." and row[i + 1] == "O":
                # print(f"{item} : {row[i+1]}")
                matrix[row_index][i], matrix[row_index][i + 1] = (
                    matrix[row_index][i + 1],
                    matrix[row_index][i],
                )
                swaps_made += 1
                continue
    if swaps_made > 0:
        return apply_lever(matrix)
    else:
        return matrix


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


@functools.lru_cache(maxsize=None)
def perform_cycle(matrix):
    matrix = tuple(map(tuple, matrix))  # Convert matrix to tuple of tuples
    matrix = apply_lever(rotate_matrix_counterclockwise(matrix))  ## north
    matrix = apply_lever(rotate_matrix_clockwise(matrix))  ## west
    matrix = apply_lever(rotate_matrix_clockwise(matrix))  ## south
    matrix = apply_lever(rotate_matrix_clockwise(matrix))  ## east
    # return to north facing
    matrix = rotate_matrix_clockwise(rotate_matrix_clockwise(matrix))
    return matrix


def main():
    matrix = process_input(read_input())

    for i in range(1000000000):
        matrix = perform_cycle(tuple(map(tuple, matrix)))
        print(i)

    total = 0
    for i, row in enumerate(reversed(matrix)):
        total += row.count("O") * (i + 1)
    print(total)


if __name__ == "__main__":
    main()
