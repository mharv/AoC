def load_input(path):
    return open(path, "r")


def check(line):
    line = line.strip().split(" ")
    line = [int(i) for i in line]
    # print(line)
    flag = None
    for i, num in enumerate(line):
        if i == 0:
            continue
        if line[i] > line[i - 1]:
            if flag and flag != "increasing":
                return False
            else:
                flag = "increasing"
        else:
            if flag and flag != "decreasing":
                return False
            else:
                flag = "decreasing"

        if abs(line[i] - line[i - 1]) > 3 or abs(line[i] - line[i - 1]) < 1:
            return False
    return True


def main():
    input_file = load_input("./input.txt")
    # input_file = load_input("./test_input.txt")
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
