def get_input(file_path: String) -> List[String]:
    var  f = open(file_path, "r")
    inputs = List[String]()
    for line in f.read().splitlines():
        inputs.append(String(line))
    f.close()
    return inputs^


def process_row(row: String) -> Tuple[String, Int]:
    var direction = row[0]
    var distance = row[1:len(row)]
    var result = (String(direction), atol(distance))
    return result

def day_1():
    current_position = 50
    counter = 0 
    
    # var input_lines = get_input("./example-input.txt")
    var input_lines = get_input("./input.txt")
    for line in input_lines:
        direction, distance = process_row(line)
        if direction == "L":
            current_position -= distance
        elif direction == "R":
            current_position += distance
        
        if current_position < 0:
            current_position = current_position % 100
        elif current_position >= 100:
            current_position = current_position % 100

        if current_position == 0:
            counter += 1
    print("Final counter:", counter)

def day_2():
    pass

def main():
    day_1()
    # day_2()
    