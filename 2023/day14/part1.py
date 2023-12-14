# Description: Advent of Code: Day 14, 2023
# Created by: Mitchell Harvey

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
                print(f"{item} : {row[i+1]}")
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
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    matrix = rotate_matrix_clockwise(
        apply_lever(rotate_matrix_counterclockwise(process_input(read_input())))
    )

    total = 0
    for i, row in enumerate(reversed(matrix)):
        total += row.count("O") * (i + 1)
    print(total)


if __name__ == "__main__":
    main()
