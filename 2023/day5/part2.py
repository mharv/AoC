# Description: Advent of Code: Day 5, 2023
# Created by: Mitchell Harvey

example = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""


class ConversionMap:
    def __init__(self, name):
        self.name = name
        self.maps = []

    def process(self, value):
        # 0: destination, 1: source, 2: range
        for map in self.maps:
            if value in range(map[1], map[1] + map[2]):
                return value + (map[0] - map[1])
        return value


def create_maps_seeds(input):
    seeds = []
    maps = []
    for i, line in enumerate(input.splitlines()):
        temp_map = ConversionMap("temp")
        if line == "":
            continue
        if i == 0:
            seed_string = line.split(": ")[1].split()
            for seed_index in range(0, len(seed_string), 2):
                # print(seed_string[seed_index])
                # print(seed_string[seed_index + 1])
                for seed_no in range(
                    int(seed_string[seed_index]),
                    int(seed_string[seed_index]) + int(seed_string[seed_index + 1]),
                ):
                    seeds.append(int(seed_no))

        else:
            if ":" in line:
                temp_map.name = line.split()[0]
                maps.append(temp_map)
            else:
                # map construction logic
                maps[len(maps) - 1].maps.append(
                    (int(line.split()[0]), int(line.split()[1]), int(line.split()[2]))
                )

    return seeds, maps


def read_input():
    return example
    with open("./input.txt") as file:
        return file.read()


def main():
    seeds, maps = create_maps_seeds(read_input())
    # print(seeds)
    targets = []
    for seed in seeds:
        target = seed
        for map in maps:
            target = map.process(target)
        targets.append(target)
    # print(targets)
    print(min(targets))


if __name__ == "__main__":
    main()
