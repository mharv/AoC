# Description: Advent of Code: Day 8, 2023
# Created by: Mitchell Harvey

example = """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"""

mapp = {}


def process_input(input):
    instructions = list(input.splitlines()[0].replace("R", "1").replace("L", "0"))
    mapp = {}
    for i, line in enumerate(input.splitlines()):
        if i == 0 or i == 1:
            continue
        key = line.split("=")[0].strip()
        value = (
            line.split("=")[1].replace("(", "").replace(")", "").strip().split(", ")[0],
            line.split("=")[1].replace("(", "").replace(")", "").strip().split(", ")[1],
        )
        mapp[key] = value

    return instructions, mapp


# def apply_instructions(instructions, mapp, count, current_position):
#     new_position = mapp[current_position][int(instructions[count % len(instructions)])]
#     if new_position != "ZZZ":
#         apply_instructions(instructions, mapp, count + 1, new_position)
#     else:
#         print(count + 1)


def apply_instructions(instructions, mapp, current_position):
    count = 0
    while True:
        new_position = mapp[current_position][
            int(instructions[count % len(instructions)])
        ]
        if new_position != "ZZZ":
            current_position = new_position
            count += 1
        else:
            print(count + 1)
            break


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    instructions, mapp = process_input(read_input())
    start_position = list(mapp.keys())[0]
    # print(mapp)
    apply_instructions(instructions, mapp, start_position)


if __name__ == "__main__":
    main()
