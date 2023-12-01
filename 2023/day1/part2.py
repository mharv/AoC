# Description: Advent of Code: Day 1, 2023
# Created by: Mitchell Harvey

example = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

lookup = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def check_start(line):
    queue = ""
    for char in list(line):
        if char.isnumeric():
            return ""
        queue += char

        new_start = ""
        if len(queue) >= 3:
            for key in lookup:
                if key in queue:
                    new_start += lookup[key]
                    return new_start


def check_end(line):
    queue = ""
    for char in reversed(list(line)):
        if char.isnumeric():
            return ""
        queue = char + queue

        new_end = ""
        if len(queue) >= 3:
            for key in lookup:
                if key in queue:
                    new_end += lookup[key]
                    return new_end


def main():
    total = 0
    for line in read_input().splitlines():
        stripped_line = check_start(line) + line
        stripped_line = stripped_line + check_end(stripped_line)

        stripped_line = list(filter(lambda x: x.isnumeric(), list(stripped_line)))
        total += int(stripped_line[0] + stripped_line[-1])
    print(total)


if __name__ == "__main__":
    main()
