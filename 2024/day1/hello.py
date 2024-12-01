def load_input(path):
    return open(path, "r")


def main():

    puzzle_input = load_input("./input.txt")
    first_list = []
    second_list = []
    for line in puzzle_input:
        numbers = line.strip().split("   ")
        assert len(numbers) == 2
        assert numbers[0].isnumeric
        assert numbers[1].isnumeric
        first_list.append(int(numbers[0]))
        second_list.append(int(numbers[1]))

    first_list.sort()
    second_list.sort()

    diffs = []
    for i, j in zip(first_list, second_list):
        diffs.append(abs(i - j))

    print(sum(diffs))


if __name__ == "__main__":
    main()
