def get_input(file_path: String) -> List[List[String]]:
    var  f = open(file_path, "r")
    inputs = List[List[String]]()
    for line in f.read().splitlines():
        temp_line = List[String]()
        for i in range(len(line)):
            temp_line.append(line[i])
        inputs.append(temp_line^)
    f.close()
    return inputs^

def create_empty_2d_list(rows: Int, cols: Int) -> List[List[Int]]:
    var grid = List[List[Int]]()
    for _ in range(rows):
        var row = List[Int]()
        for _ in range(cols):
            row.append(0)
        grid.append(row^)
    return grid^


def part1():
    var input_file = "input.txt"
    var inputs = get_input(input_file)
    var grand_total = 0
    
    for i, line in enumerate(inputs):
        for j, char in enumerate(line):
            if char == "@":
                var total = 0
                # check each adjacent character if in bounds
                if i > 0:
                    if inputs[i-1][j] == "@":
                        total += 1
                if i < len(inputs) - 1:
                    if inputs[i+1][j] == "@":
                        total += 1
                if j > 0:
                    if inputs[i][j-1] == "@":
                        total += 1
                if j < len(line) - 1:
                    if inputs[i][j+1] == "@":
                        total += 1     
                if i > 0 and j > 0:
                    if inputs[i-1][j-1] == "@":
                        total += 1
                if i > 0 and j < len(line) - 1:
                    if inputs[i-1][j+1] == "@":
                        total += 1
                if i < len(inputs) - 1 and j > 0:
                    if inputs[i+1][j-1] == "@":
                        total += 1
                if i < len(inputs) - 1 and j < len(line) - 1:
                    if inputs[i+1][j+1] == "@":
                        total += 1
                if total < 4:
                    grand_total += 1


                
    print(grand_total)


def part2():
    var input_file = "input.txt"
    var inputs = get_input(input_file)
    var returned = 1
    var grand_total = 0

    while returned != 0:
        returned = 0
        var markers = create_empty_2d_list(len(inputs), len(inputs[0]))
        for i, line in enumerate(inputs):
            for j, char in enumerate(line):
                if char == "@":
                    var total = 0
                    # check each adjacent character if in bounds
                    if i > 0:
                        if inputs[i-1][j] == "@":
                            total += 1
                    if i < len(inputs) - 1:
                        if inputs[i+1][j] == "@":
                            total += 1
                    if j > 0:
                        if inputs[i][j-1] == "@":
                            total += 1
                    if j < len(line) - 1:
                        if inputs[i][j+1] == "@":
                            total += 1     
                    if i > 0 and j > 0:
                        if inputs[i-1][j-1] == "@":
                            total += 1
                    if i > 0 and j < len(line) - 1:
                        if inputs[i-1][j+1] == "@":
                            total += 1
                    if i < len(inputs) - 1 and j > 0:
                        if inputs[i+1][j-1] == "@":
                            total += 1
                    if i < len(inputs) - 1 and j < len(line) - 1:
                        if inputs[i+1][j+1] == "@":
                            total += 1
                    if total < 4:
                        grand_total += 1
                        returned += 1
                        markers[i][j] = 1
        for i in range(len(inputs)):
            for j in range(len(inputs[0])):
                if markers[i][j] == 1:
                    inputs[i][j] = "."

                
    print(grand_total)

def main():
    # part1()
    part2()
