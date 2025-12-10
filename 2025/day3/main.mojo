def get_input(file_path: String) -> List[String]:
    var  f = open(file_path, "r")
    inputs = List[String]()
    for line in f.read().splitlines():
        inputs.append(String(line))
    f.close()
    return inputs^

def get_values(line: String) -> List[Int]:
    var values = List[Int]()
    for val in line:
        values.append(Int(val))
    return values^

# def repeat_check(s: String, divisor: Int) -> Bool:
#     length = len(s)
#     if length % divisor != 0:
#         return False
#     return s[0:length//divisor] == s[length//divisor:length]


# def repeat_check_recursive(s: String, divisor: Int) -> Bool:
#     length = len(s)
#     # Try divisors starting at the provided one and increasing.
#     # For each divisor we require that each chunk length is at least 2 characters.
#     for d in range(divisor, length + 1):
#         if length % d != 0:
#             continue
#         part_len = length // d
#         if part_len < 1:
#             # stop increasing d further because further divisors will only make part_len smaller
#             break
#         ok = True
#         for i in range(1, d):
#             if s[0:part_len] != s[i*part_len:(i+1)*part_len]:
#                 ok = False
#                 break
#         if ok:
#             return True
#     return False


def part_1():
    var input_lines = get_input("./input.txt")
    var total = 0
    for line in input_lines:
        var values = get_values(line)
        var max_value = 0
        for i, v in enumerate(values):
            for j in range(i + 1, len(values)):
                var v2 = values[j]
                if (v*10) + v2 > max_value:
                    max_value = (v*10) + v2
        total += max_value
    print(total)

def part_2():
    var input_lines = get_input("./input.txt")
    var total = 0
    for line in input_lines:
        var values = get_values(line)
        var n = len(values)
        var k = 12
        if n < k:
            continue  # skip lines with fewer than 12 digits
        var selected = List[Int]()
        var start = 0
        for pos in range(k):
            # For each position, find the max digit in the allowed range
            max_digit = -1
            max_idx = -1
            # The furthest we can go is n - (k - pos)
            for i in range(start, n - (k - pos) + 1):
                if values[i] > max_digit:
                    max_digit = values[i]
                    max_idx = i
            selected.append(max_digit)
            start = max_idx + 1
        # Build the number from selected digits
        var num = 0
        for d in selected:
            num = num * 10 + d
        total += num
    print(total)


def main():
    # part_1()
    part_2()
    