from collections import Set

def get_input(file_path: String) -> Tuple[List[Tuple[Int, Int]], List[Int]]:
    var  f = open(file_path, "r")
    var ranges = List[Tuple[Int, Int]]()
    var inputs = List[Int]()
    for line in f.read().splitlines():
        var lineString = String(line)
        if lineString.find("-") != -1:
            var parts = lineString.split("-")
            var start = Int(parts[0])
            var end = Int(parts[1])
            ranges.append((start, end))
        elif lineString == "":
            # skip blank lines
            continue
        else:
            inputs.append(Int(lineString))
    f.close()
    return (ranges^, inputs^)

def part1():
    # var result = get_input("./example-input.txt")
    var result = get_input("./input.txt")
    var count = 0

    for i in range(len(result[1])):
        var r = result[1][i]
        print("Checking number:", r)
        for j in range(len(result[0])):
            var range_pair = result[0][j]
            if r >= range_pair[0] and r <= range_pair[1]:
                print("  Value", r, "is within range")
                count += 1
                break
            else:
                print("  Value", r, "is outside range")
    print(count)

def part2(): 
    var result = get_input("./input.txt")
    
    # Sort ranges by start position
    var sorted_ranges = result[0].copy()
    # TODO: Implement sorting (Mojo doesn't have built-in sort yet)
    for i in range(len(sorted_ranges)):
        for j in range(i + 1, len(sorted_ranges)):
            if sorted_ranges[i][0] > sorted_ranges[j][0]:
                var temp = sorted_ranges[i]
                sorted_ranges[i] = sorted_ranges[j]
                sorted_ranges[j] = temp
    
    # Merge overlapping ranges
    var merged = List[Tuple[Int, Int]]()
    merged.append(sorted_ranges[0])
    
    for i in range(1, len(sorted_ranges)):
        var current = sorted_ranges[i]
        var last = merged[len(merged) - 1]
        
        if current[0] <= last[1] + 1:
            # Overlapping or adjacent - merge
            var new_end = max(last[1], current[1])
            merged[len(merged) - 1] = (last[0], new_end)
        else:
            # No overlap - add new range
            merged.append(current)
    
    # Count total numbers in merged ranges
    var count = 0
    
    print(len(merged))
    for i in range(len(merged)):
        print("Merged range:", merged[i][0], "-", merged[i][1])
        
        count += merged[i][1] - merged[i][0] + 1
        print("Count so far:", count)
    
    print(count)





def main():
    # part1()
    part2()
    