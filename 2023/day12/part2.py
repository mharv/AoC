# Description: Advent of Code: Day 12, 2023
# Created by: Mitchell Harvey

example = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""


def process_input(input):
    return input


def apply_instructions(instructions):
    print(instructions)


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


def main():
    print(read_input())


if __name__ == "__main__":
    main()
