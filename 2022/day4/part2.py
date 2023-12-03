# Description: Advent of Code: Day 4, 2022
# Created by: Mitchell Harvey

example = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""


def process_input(line):
    pair = line.split(",")
    first_start = int(pair[0].split("-")[0])
    second_start = int(pair[1].split("-")[0])
    first_end = int(pair[0].split("-")[1])
    second_end = int(pair[1].split("-")[1])

    if first_start > second_end:
        return False
    if second_start > first_end:
        return False
    return True


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    input = read_input()
    result = 0
    for line in input.splitlines():
        if process_input(line):
            result += 1
    print(result)


if __name__ == "__main__":
    main()
