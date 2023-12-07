# Description: Advent of Code: Day 7, 2023
# Created by: Mitchell Harvey

example = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""


strength = {
    "A": 13,
    "K": 12,
    "Q": 11,
    "J": 10,
    "T": 9,
    "9": 8,
    "8": 7,
    "7": 6,
    "6": 5,
    "5": 4,
    "4": 3,
    "3": 2,
    "2": 1,
}


def process_input(input):
    hands = []

    for line in input.splitlines():
        hands.append([line.split()[0], line.split()[1]])

    return hands


def order_by_power(hand):
    temp_dict = {}
    for char in list(hand[0]):
        if char in temp_dict:
            temp_dict[char] += 1
        else:
            temp_dict[char] = 1

    print(temp_dict)

    if len(temp_dict) == 1:
        hand.append(7)  # "5OK"
    if len(temp_dict) == 2:
        if list(temp_dict.values())[0] == 4 or list(temp_dict.values())[1] == 4:
            hand.append(6)  # "4OK"
        else:
            hand.append(5)  # "FH"
    if len(temp_dict) == 3:
        if (
            list(temp_dict.values())[0] == 3
            or list(temp_dict.values())[1] == 3
            or list(temp_dict.values())[2] == 3
        ):
            hand.append(4)  # "3OK"
        else:
            hand.append(3)  # "2P"
    if len(temp_dict) == 4:
        hand.append(2)  # "2OK"
    if len(temp_dict) == 5:
        hand.append(1)  # 1OK

    return hand


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def main():
    ordered = []
    print(process_input(read_input()))

    for hand in process_input(read_input()):
        hand = order_by_power(hand)
        ordered.append(tuple(hand))

    ordered.sort(key=lambda x: x[2])
    print(ordered)

    new_ordered = []
    for i in range(1, 8):
        filtered_array = [t for t in ordered if t[2] == i]
        if len(filtered_array) > 0:
            filtered_array = sorted(filtered_array, key=lambda x: x[2])
            new_ordered.append(filtered_array)

    print(new_ordered)

    def custom_sort_key(s):
        return tuple(strength.get(char) for char in s[0])

    new_ordered_list = []
    for group in new_ordered:
        if len(group) > 0:
            new_ordered_list.append(sorted(group, key=custom_sort_key))
    print(new_ordered_list)

    flattened_list = [item for sublist in new_ordered_list for item in sublist]
    print(flattened_list)
    total = 0
    for i, hand in enumerate(flattened_list):
        total = total + ((i + 1) * int(hand[1]))
    print(total)


if __name__ == "__main__":
    main()
