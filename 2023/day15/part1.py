# Description: Advent of Code: Day 14, 2023
# Created by: Mitchell Harvey

example = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"""


def process_input(input):
    return input


def apply_instructions(input):
    total = 0
    for i in input:
        no = ord(i)
        total += no
        total *= 17
        total %= 256
    print(total)
    return total


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    total = 0
    for i in read_input().strip().split(","):
        total += apply_instructions(i)
    print(total)


if __name__ == "__main__":
    main()
