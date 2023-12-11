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
        input.insert(i + row_count, "*" * len(input[0]))
        input = "\n".join(input)
        row_count += 1

    col_count = 0
    for i in cols_to_expand:
        input = input.splitlines()
        for j, line in enumerate(input):
            input[j] = line[: i + col_count] + "*" + line[i + col_count :]
        input = "\n".join(input)
        col_count += 1
    return input


def get_coordinates(input):
    coordinates = []
    for i, line in enumerate(input.splitlines()):
        for j, char in enumerate(line):
            if char == "#":
                coordinates.append((i, j))
    return coordinates


def find_all_unique_pairs(coordinates):
    pairs = []
    for i in range(len(coordinates)):
        for j in range(i + 1, len(coordinates)):
            pairs.append((coordinates[i], coordinates[j]))
    return pairs


def get_distance_between_pair(pair):
    return abs(pair[0][0] - pair[1][0]) + abs(pair[0][1] - pair[1][1])


def get_distances_between_pairs(pairs, multiplier_lines):
    distances = []
    size_of_galazy = 1000000 - 2
    for pair in pairs:
        distance = get_distance_between_pair(pair)
        for line in multiplier_lines:
            if line[1] > min(pair[0][1], pair[1][1]) and line[1] < max(
                pair[0][1], pair[1][1]
            ):
                distance += size_of_galazy
        for line in multiplier_lines:
            if line[0] > min(pair[0][0], pair[1][0]) and line[0] < max(
                pair[0][0], pair[1][0]
            ):
                distance += size_of_galazy

        distances.append(distance)
    return distances


def find_multiplier_lines(input):
    multiplier_lines = []
    for i, line in enumerate(input.splitlines()):
        if all(char == "*" for char in line):
            multiplier_lines.append((i, 0))
    for i in range(len(input.splitlines()[0])):
        column = ""
        for line in input.splitlines():
            column += line[i]
        if all(char == "*" for char in column):
            multiplier_lines.append((0, i))
    return multiplier_lines


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input = read_input()
    input = expand(input)
    for line in input.splitlines():
        print(line)
    coordinates = get_coordinates(input)
    pairs = find_all_unique_pairs(coordinates)
    multiplier_lines = find_multiplier_lines(input)
    print(sum(get_distances_between_pairs(pairs, multiplier_lines)))


if __name__ == "__main__":
    main()
