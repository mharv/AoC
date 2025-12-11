

def get_input(file_path: String) -> List[List[String]]:
    var f = open(file_path, "r")
    var result = List[List[String]]()
    for i in f.read().splitlines():
        temp = List[String]()
        for c in i.split(" "):
            if not String(c).strip() == "":
                temp.append(String(c))
        result.append(temp^)
    return result^


def get_input_p2(file_path: String) -> List[List[String]]:
    var f = open(file_path, "r")
    var result = List[List[String]]()
    for i in f.read().splitlines():
        temp = List[String]()
        for c in i:
            # if not String(c) == "":
            temp.append(String(c))
        result.append(temp^)
    return result^


def part1():
    var input_content = get_input("./input.txt")

    var values = List[List[Int]]()

    var total = 0
    
    for i in range(0, len(input_content)):
        if i != len(input_content) - 1:
            # Initialize columns if this is the first row
            if i == 0:
                for _ in range(0, len(input_content[i])):
                    values.append(List[Int]())
            
            # Add each value to its corresponding column
            for j in range(0, len(input_content[i])):
                values[j].append(Int(input_content[i][j]))
        else:
            for j in range(0, len(input_content[i])):
                if input_content[i][j] == "+":
                    var prod = 0
                    for k in range(0, len(values[j])):
                        prod += values[j][k]
                        print(prod)

                    total += prod
                if input_content[i][j] == "*":
                    var prod = 1
                    for k in range(0, len(values[j])):
                        prod *= values[j][k]
                        print(prod)
                    total += prod
                print("Total so far:", total)
    
    print(total)

def part2():
    var input_content = get_input_p2("./input.txt")

    var total = 0
    input_content.reverse()
    var operators = List[String]()
    
    for i in range(len(input_content[0])):
        if input_content[0][i] == "+" or input_content[0][i] == "*":
            operators.append(input_content[0][i])


    operators.reverse()
    var numbers = List[List[String]]()

    for i in range(1, len(input_content)):
        temp = input_content[i].copy()
        temp.reverse() 
        numbers.append(temp.copy())
        print(temp)
    
    var numbers_processed = List[List[Int]]()

    var temp_int_list = List[Int]()
    for i in range(len(numbers[0])):
        var temp_list = List[String]()
        for j in range(len(numbers)):
            temp_list.append(numbers[j][i])
        if len("".join(temp_list).strip()) != 0:
            temp_list.reverse()
            temp_int_list.append(Int("".join(temp_list).strip()))
        else:
            numbers_processed.append(temp_int_list.copy())
            temp_int_list = List[Int]()
    numbers_processed.append(temp_int_list.copy())

    for j in range(0, len(operators)):
        if operators[j] == "+":
            var prod = 0
            for k in range(0, len(numbers_processed[j])):
                prod += numbers_processed[j][k]
                print(prod)
            print("Adding", prod)

            total += prod
        if operators[j] == "*":
            var prod = 1
            for k in range(0, len(numbers_processed[j])):
                prod *= numbers_processed[j][k]
                print(prod)
            print("Multiplying", prod)
            total += prod
    
    print(total)
    
    

def main():
    # part1()
    part2()