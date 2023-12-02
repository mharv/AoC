# Description: Advent of Code: Day 2, 2023
# Created by: Mitchell Harvey

example = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""


limit_lookup = {
    "red": 12,
    "green": 13,
    "blue": 14,
}


def prepare_string(line):
    # prepare string returns a game number and a dict with color : number
    result = {"game_number": 0, "result_maps": []}

    line = line.split(":")
    result["game_number"] = int(line[0].split(" ")[1])

    reveal_sets = line[1].split(";")
    for reveals in reveal_sets:
        game_result = {
            "red": 0,
            "green": 0,
            "blue": 0,
        }
        reveals = reveals.split(",")
        for reveal in reveals:
            for color in game_result:
                if color in reveal:
                    reveal = reveal.strip().split(" ")
                    game_result[reveal[1]] = int(reveal[0])
        result["result_maps"].append(game_result)
    return result


def do_power_check(game):
    outcome = {
        "red": 0,
        "green": 0,
        "blue": 0,
    }

    for result in game["result_maps"]:
        for color in outcome:
            if result[color] > outcome[color]:
                outcome[color] = result[color]

    power_calc = 1
    for _, v in outcome.items():
        power_calc = power_calc * v

    return power_calc


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    total = 0
    input = read_input()
    games = []
    for line in input.splitlines():
        games.append(prepare_string(line))
    for game in games:
        total += do_power_check(game)
    print(total)


if __name__ == "__main__":
    main()
