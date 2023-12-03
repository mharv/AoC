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
...$..*...
.664...598"""
# above gives 13(7*2 + 5 + 15) = 442


class Cell:
    def __init__(self, value, gears=[]):
        self.value = value
        self.gears = []

    def __str__(self):
        return f"{self.value}: {self.gears}"


def create_overlaps(input):
    return 0


def return_overlaps(input, overlaps):
    return 0


def label_cell_gears(matrix, input):
    gear_number = 0
    width = len(input[0]) - 1
    height = len(input) - 1
    for i, line in enumerate(input.splitlines()):
        for j, char in enumerate(line):
            if char == "*":
                gear_number += 1
                if len(matrix[i][j].gears) == 0:
                    matrix[i][j].gears.append(gear_number)

                if matrix[i][j - 1].value != "*":
                    matrix[i][j - 1].gears.append(gear_number)
                if matrix[i][j + 1].value != "*":
                    matrix[i][j + 1].gears.append(gear_number)
                if matrix[i - 1][j].value != "*":
                    matrix[i - 1][j].gears.append(gear_number)
                if matrix[i + 1][j].value != "*":
                    matrix[i + 1][j].gears.append(gear_number)

                if matrix[i - 1][j - 1].value != "*":
                    matrix[i - 1][j - 1].gears.append(gear_number)
                if matrix[i - 1][j + 1].value != "*":
                    matrix[i - 1][j + 1].gears.append(gear_number)
                if matrix[i + 1][j - 1].value != "*":
                    matrix[i + 1][j - 1].gears.append(gear_number)
                if matrix[i + 1][j + 1].value != "*":
                    matrix[i + 1][j + 1].gears.append(gear_number)
    return matrix, gear_number


def convert_to_matrix(input):
    output = []
    for i, line in enumerate(input.splitlines()):
        output.append([])
        for j, char in enumerate(line):
            output[i].append(Cell(char))
    return output


def print_matrix_if_gear_no(matrix, gear_no):
    for line in matrix:
        full_line = ""
        for cell in line:
            if gear_no in cell.gears:
                full_line += str(cell.value)
            else:
                full_line += "."
        # print(full_line)
        print(f"\n")


def print_matrix(matrix):
    for line in matrix:
        full_line = ""
        for cell in line:
            print(cell)
        print(f"\n")


def prefill_number_cells_with_gear_no(matrix, input):
    temp_coordinates = []
    for i, line in enumerate(input.splitlines()):
        for j, char in enumerate(line):
            # if j == len(line) - 1:
            if char.isnumeric():
                temp_coordinates.append((i, j))
            if (not char.isnumeric() or j == len(line) - 1) and len(
                temp_coordinates
            ) > 0:
                # end of number, consolidate all gears for all cell coordinates
                temp_gear_numbers = []
                for coord in temp_coordinates:
                    for gear_no in matrix[coord[0]][coord[1]].gears:
                        if not gear_no in temp_gear_numbers:
                            temp_gear_numbers.append(gear_no)
                if len(temp_gear_numbers) > 0:
                    for coord in temp_coordinates:
                        matrix[coord[0]][coord[1]].gears = temp_gear_numbers[:]
                    temp_gear_numbers = []
                    temp_coordinates = []
                else:
                    temp_coordinates = []
    return matrix


def get_result_if_gear_no(matrix, gear_no):
    result = 0
    number_strings = []

    for line in matrix:
        full_number = ""
        for i, cell in enumerate(line):
            if gear_no in cell.gears and cell.value.isnumeric():
                full_number += cell.value
            if (not cell.value.isnumeric() or i == len(line) - 1) and len(
                full_number
            ) > 0:
                number_strings.append(int(full_number))
                full_number = ""

    print(number_strings)
    if len(number_strings) == 2:
        result = number_strings[0] * number_strings[1]
    return result


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input_converted = convert_to_matrix(read_input())
    matrix, gear_count = label_cell_gears(input_converted, read_input())
    total = 0

    matrix = prefill_number_cells_with_gear_no(matrix, read_input())
    print_matrix(matrix)

    all_multiplied_results = []
    for i in range(1, gear_count + 1):
        # print_matrix_if_gear_no(matrix, i)
        print(get_result_if_gear_no(matrix, i))
        all_multiplied_results.append(get_result_if_gear_no(matrix, i))
        print("\n")
    print(sum(all_multiplied_results))


if __name__ == "__main__":
    main()
