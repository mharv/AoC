def load_input(path):
    return open(path, "r")


def check(line):
    line = line.strip().split(" ")
    line = [int(i) for i in line]
    flag = None
    collect = []
    for exclude in range(len(line)):
        for i, num in enumerate(line):
            if i == 0 or i == exclude:
                continue
            if line[i] > line[i - 1]:
                if flag and flag != "increasing":
                    collect.append(False)
                    break
                else:
                    flag = "increasing"
            else:
                if flag and flag != "decreasing":
                    collect.append(False)
                    break
                else:
                    flag = "decreasing"

            if abs(line[i] - line[i - 1]) > 3 or abs(line[i] - line[i - 1]) < 1:
                collect.append(False)
                break
        if len(collect) == 0:
            collect.append(True)

    print(f"{collect}")

    return True in collect


def main():
    # input_file = load_input("./input.txt")
    input_file = load_input("./test_input.txt")
    puzzle = []
    for line in input_file:
        puzzle.append(line)

    # assert len(puzzle) == 1000

    result = 0

    for level in puzzle:
        if check(level):
            print(level)
            result += 1

    print(result)
    print("Done.")


if __name__ == "__main__":
    main()
