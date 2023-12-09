# Description: Advent of Code: Day 9, 2023
# Created by: Mitchell Harvey

example = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""


def process_input(line):
    output = []
    for number in line.split(" "):
        output.append(int(number))

    return output


def apply_instructions(processed):
    full_array = []
    full_array.append(processed)
    count = 0
    while not all(x == 0 for x in full_array[len(full_array) - 1]):
        temp_array = []
        for i, x in enumerate(full_array[count]):
            if i == len(full_array[count]) - 1:
                continue
            temp_array.append(full_array[count][i + 1] - x)
        full_array.append(temp_array)
        count += 1
    return full_array


def find_next_value(processed):
    processed = list(reversed(processed))
    for i, line in enumerate(processed):
        if i == 0:
            processed[i].append(0)
        else:
            processed[i].append(
                line[len(line) - 1] + processed[i - 1][len(processed[i - 1]) - 1]
            )
    return processed[len(processed) - 1][-1]


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    numbers = []
    for line in read_input().splitlines():
        numbers.append(find_next_value(apply_instructions(process_input(line))))
    print(sum(numbers))


if __name__ == "__main__":
    main()
