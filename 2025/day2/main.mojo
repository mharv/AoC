def get_input(file_path: String) -> List[String]:
    var  f = open(file_path, "r")
    inputs = List[String]()
    for line in f.read().splitlines():
        inputs.append(String(line))
    f.close()
    return inputs^

def get_ranges(lines: List[String]) -> List[Tuple[Int, Int]]:
    var ranges = List[Tuple[Int, Int]]()
    for line in lines:
        for range in line.split(","):
            var parts = range.split("-")
            ranges.append((Int(parts[0]), Int(parts[1])))
    return ranges^

def repeat_check(s: String) -> Bool:
    length = len(s)
    if length % 2 != 0:
        return False
    return s[0:length//2] == s[length//2:length]

def part_1():
    var input_lines = get_input("./input.txt")
    var ranges = get_ranges(input_lines)
    var total = 0
    for item in ranges:
        print("Range: " + String(item[0]) + " to " + String(item[1]))
        for i in range(item[0], item[1] + 1):
            if repeat_check(String(i)):
                print("Found repeating number: " + String(i))
                total += i
    print("Total sum of repeating numbers: " + String(total))

def part_2():
    pass

def main():
    part_1()
    # part_2()
    