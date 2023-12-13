# Description: Advent of Code: Day 13, 2023
# Created by: Mitchell Harvey

example = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""


print_matrix = lambda matrix: print("\n".join([" ".join(line) for line in matrix]))


def process_input(input):
    processed = []
    temp_matrix = []
    for i, line in enumerate(input.split("\n")):
        if i == len(input.split("\n")) - 1 and line != "":
            temp_matrix.append(list(line))
            processed.append(temp_matrix)
            break
        if line == "":
            processed.append(temp_matrix)
            temp_matrix = []
        else:
            temp_matrix.append(list(line))

    return processed


def check_for_reflection(matrix):
    reflections = []
    # reflections [X, Y] where X is the row number and Y is the number of reflections

    for i, line in enumerate(matrix):
        temp_reflection = [0, 0]
        if i == len(matrix) - 1:
            break
        if line == matrix[i + 1]:
            temp_reflection[0] = i
            temp_reflection[1] += 1
            # if there is a match, check equality of the all previous rows and next rows
            for l, r in zip(reversed(matrix[:i]), matrix[i + 2 :]):
                if l == r:
                    print(l, r)
                    temp_reflection[1] += 1
                else:
                    break

            if (
                temp_reflection[0] - (temp_reflection[1] - 1) == 0
                or temp_reflection[0] + temp_reflection[1] == len(matrix) - 1
            ):
                reflections.append(temp_reflection)
            # reflections.append(temp_reflection)
    return reflections


def get_score(reflections):
    # get reflection with the highest score
    if len(reflections) == 0:
        return 0
    highest_score = 0
    row_number = 0
    reflection_index = 0
    for i, reflection in enumerate(reflections):
        if reflection[1] > highest_score:
            highest_score = reflection[1]
            row_number = reflection[0]
            reflection_index = i
    if reflections[reflection_index][2] == "vertical":
        return (row_number + 1) * 100
    if reflections[reflection_index][2] == "horizontal":
        return row_number + 1


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def transpose(matrix):
    try:
        return [
            [matrix[j][i] for j in range(len(matrix))] for i in range(len(matrix[0]))
        ]
    except:
        return IndexError


def rotate_matrix(matrix):
    try:
        return [
            [matrix[j][i] for j in range(len(matrix) - 1, -1, -1)]
            for i in range(len(matrix[0]))
        ]
    except:
        return IndexError


def main():
    totals = []
    for matrix in process_input(read_input()):
        # print(matrix)
        print_matrix(matrix)
        vertical_reflections = check_for_reflection(matrix)
        if len(vertical_reflections) > 0:
            for i, reflection in enumerate(vertical_reflections):
                vertical_reflections[i].append("vertical")

        print("\n")

        transposed_matrix = rotate_matrix(matrix)
        print_matrix(transposed_matrix)
        horizontal_reflections = check_for_reflection(transposed_matrix)
        if len(horizontal_reflections) > 0:
            for i, reflection in enumerate(horizontal_reflections):
                horizontal_reflections[i].append("horizontal")

        print(vertical_reflections)
        print(horizontal_reflections)
        score = get_score(vertical_reflections + horizontal_reflections)
        print(score)
        totals.append(score)

    print(sum(totals))


if __name__ == "__main__":
    main()
