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

def repeat_check(s: String, divisor: Int) -> Bool:
    length = len(s)
    if length % divisor != 0:
        return False
    return s[0:length//divisor] == s[length//divisor:length]


def repeat_check_recursive(s: String, divisor: Int) -> Bool:
    length = len(s)
    # Try divisors starting at the provided one and increasing.
    # For each divisor we require that each chunk length is at least 2 characters.
    for d in range(divisor, length + 1):
        if length % d != 0:
            continue
        part_len = length // d
        if part_len < 1:
            # stop increasing d further because further divisors will only make part_len smaller
            break
        ok = True
        for i in range(1, d):
            if s[0:part_len] != s[i*part_len:(i+1)*part_len]:
                ok = False
                break
        if ok:
            return True
    return False


def part_1():
    var input_lines = get_input("./input.txt")
    var ranges = get_ranges(input_lines)
    var total = 0
    for item in ranges:
        print("Range: " + String(item[0]) + " to " + String(item[1]))
        for i in range(item[0], item[1] + 1):
            if repeat_check(String(i), 2):
                print("Found repeating number: " + String(i))
                total += i
    print("Total sum of repeating numbers: " + String(total))

def part_2():
    var input_lines = get_input("./input.txt")
    var ranges = get_ranges(input_lines)
    var total = 0
    for item in ranges:
        print("Range: " + String(item[0]) + " to " + String(item[1]))
        for i in range(item[0], item[1] + 1):
            if repeat_check_recursive(String(i), 2):
                print("Found repeating number: " + String(i))
                total += i
    print("Total sum of repeating numbers: " + String(total))


def main():
    part_1()
    part_2()
    