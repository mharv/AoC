# Description: Advent of Code: Day 12, 2023
# Created by: Mitchell Harvey

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
                list(line.split(" ")[0]),
                [int(x) for x in line.split(" ")[1].split(",")],
            ]
        )
    for row in processed_input:
        # pad row[0] with max(row[1]) "."s both ends
        row[0] = row[0] + ["."] * max(row[1])
    return processed_input


def find_combos(row, order, count=0):
    count = 0
    # print(row)
    print(order)

    for i, item in enumerate(row):
        if item == ".":
            continue
        if i + order[0] > len(row):
            break

        if i > 0:
            # check backwards to see if any other # exists
            if "#" in row[0:i]:
                continue

        if not "." in row[i : i + order[0]]:
            print(row)
            print(f"possible fit at {i}")
            print(row[i : i + order[0]])
            # if item == "?" or item == "#":
            if i > 0:
                # check if first_item_window can fit here, and that no neighbours are "#"
                if row[i - 1] == "#" or row[i + order[0]] == "#":
                    # print(f"{order[0]} could not fit at {i}")
                    # print(item)
                    continue
                else:
                    print(f"{order[0]} fits at {i}")
                    if len(order[1:]) > 0:
                        print("into recursion")
                        count += find_combos(row[i + 1 + order[0] :], order[1:], count)

                    else:
                        count += 1

            else:
                # check if first_item_window can fit here, and that no neighbours are "#"
                if row[i + order[0]] == "#":
                    # print(f"{order[0]} could not fit at {i}")
                    # print(item)
                    continue
                else:
                    print(f"{order[0]} fits at {i}")
                    if len(order[1:]) > 0:
                        print("into recursion")
                        count += find_combos(row[i + 1 + order[0] :], order[1:], count)

                    else:
                        count += 1

    print(f"returning... count = {count}")
    return count


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


def main():
    test_case = process_input(read_input())
    # print(test_case[0])
    # print(test_case[1])

    results = []
    for test in process_input(read_input())[0:1]:
        results.append(find_combos(test[0], test[1]))

    print(results)
    print(sum(results))


if __name__ == "__main__":
    main()
