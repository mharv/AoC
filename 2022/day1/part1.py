# Description: Advent of Code: Day 1, 2022
# Created by: Mitchell Harvey

test_case = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""

def read_input():
    with open("./input.txt") as file:
        return file.read()
    
def get_list_of_totals(input):
    index = 0
    results = [0]
    for line in input.splitlines():
        if line == "":
            index += 1
            results.append(0)
        else :
            results[index] = results[index] + int(line)
    return results

def main():
    input = read_input()
    # totals = get_list_of_totals(input)
    totals = get_list_of_totals(input)
    print(max(totals))



if __name__ == "__main__":
    main()
