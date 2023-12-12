# Description: Advent of Code: Day 12, 2023
# Created by: Mitchell Harvey

import functools
import time

example = """.##.?#??.#.?# 2,1,1,1
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""


def process_input(input):
    processed_input = []
    for line in input.splitlines():
        processed_input.append(
            [
                (list(line.split(" ")[0]) + ["?"]) * 5,
                [int(x) for x in line.split(" ")[1].split(",")] * 5,
            ]
        )
    for row in processed_input:
        row[0] = row[0][:-1] + ["."] * max(row[1])
    return processed_input


@functools.lru_cache(maxsize=None)
def find_combos(row, order, count=0):
    count = 0
    print(f"Processing row: {row}, order: {order}, count: {count}")

    for i, item in enumerate(row):
        if item == ".":
            print(f"Skipping item at index {i} because it's a '.'")
            continue
        if i + order[0] > len(row):
            print(f"Breaking loop at index {i} because it's out of range")
            break

        if i > 0:
            if "#" in row[0:i]:
                print(f"Skipping item at index {i} because a '#' was found before it")
                continue

        if not "." in row[i : i + order[0]]:
            if i > 0:
                if row[i - 1] == "#" or row[i + order[0]] == "#":
                    print(
                        f"Skipping item at index {i} because a '#' was found next to it"
                    )
                    continue
                else:
                    if len(order[1:]) > 0:
                        print(f"Going into recursion at index {i}")
                        count += find_combos(
                            row[i + 1 + order[0] :], tuple(order[1:]), count
                        )
                    else:
                        if "#" not in row[i + order[0] :]:
                            count += 1
                        else:
                            print(
                                f"Skipping item at index {i} because a '#' was found after it"
                            )
                            continue
            else:
                if row[i + order[0]] == "#":
                    print(
                        f"Skipping item at index {i} because a '#' was found next to it"
                    )
                    continue
                else:
                    if len(order[1:]) > 0:
                        print(f"Going into recursion at index {i}")
                        count += find_combos(
                            row[i + 1 + order[0] :], tuple(order[1:]), count
                        )
                    else:
                        if "#" not in row[i + order[0] :]:
                            count += 1
                        else:
                            print(
                                f"Skipping item at index {i} because a '#' was found after it"
                            )
                            continue

    print(f"Returning count = {count}")
    return count


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    start = time.time()
    test_case = process_input(read_input())
    print(test_case[1])

    results = []
    for test in process_input(read_input()):
        result = find_combos(tuple(test[0]), tuple(test[1]))
        print(result)
        results.append(result)

    print(results)
    print(sum(results))

    end = time.time()
    print(f"Time taken: {end-start}")


if __name__ == "__main__":
    main()
