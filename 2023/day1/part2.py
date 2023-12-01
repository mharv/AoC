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


def replace_words(line):
    new_list = ""
    alpha_chars_list = ""
    for char in list(line):
        if char.isnumeric():
            new_list += char
            alpha_chars_list = ""
        if char.isalpha():
            alpha_chars_list += char
        if len(alpha_chars_list) >= 3:
            for key in lookup:
                if key in alpha_chars_list:
                    new_list += lookup[key]
                    alpha_chars_list = ""

    return new_list


def main():
    total = 0
    for line in read_input().splitlines():
        stripped_line = replace_words(line)
        total += int(stripped_line[0] + stripped_line[-1])
    print(total)


if __name__ == "__main__":
    main()
