# Description: Advent of Code: Day 8, 2023
# Created by: Mitchell Harvey

from math import lcm

example = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"""

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

def apply_instructions(instructions, mapp, current_position):
    count = 0
    while True:
        new_position = mapp[current_position][
            int(instructions[count % len(instructions)])
        ]
        if list(new_position)[2] != "Z":
            # print(count + 1)
            current_position = new_position
            count += 1
        else:
            print(count + 1)
            break
    return count + 1

        

def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    instructions, mapp = process_input(read_input())
    
    start_positions = [x for x in filter(lambda x: list(x)[2] == "A", mapp.keys())]
    print(start_positions)
    
    results = []
    for position in start_positions:
        results.append(apply_instructions(instructions, mapp, position))

    for result in results:
        print(result / len(instructions))

    print(lcm(results[0], results[1], results[2], results[3], results[4], results[5]))
    # print(lcm(results[0], results[1]))


if __name__ == "__main__":
    main()

