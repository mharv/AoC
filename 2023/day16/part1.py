# Description: Advent of Code: Day 16, 2023
# Created by: Mitchell Harvey

example = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."""


print_matrix = lambda matrix: print("\n".join(["".join(line) for line in matrix]))


class Beam:
    def __init__(self, x, y, direction, dimensions):
        self.x = x
        self.y = y
        self.direction = direction
        self.history = []
        self.history.append((x, y))
        self.limits = dimensions
        self.ready_for_destruction = False

    def move(self):
        if self.direction == "right":
            if self.x + 1 < self.limits[0]:
                self.x += 1
            else:
                self.ready_for_destruction = True
        if self.direction == "left":
            if self.x - 1 >= 0:
                self.x -= 1
            else:
                self.ready_for_destruction = True
        if self.direction == "up":
            if self.y - 1 >= 0:
                self.y -= 1
            else:
                self.ready_for_destruction = True
        if self.direction == "down":
            if self.y + 1 < self.limits[1]:
                self.y += 1
            else:
                self.ready_for_destruction = True
        if not self.ready_for_destruction and (self.x, self.y) in self.history:
            self.ready_for_destruction = True
        else:
            self.history.append((self.x, self.y))


def process_input(input):
    output = []
    for line in input.splitlines():
        output.append(list(line))
    return output


def apply_instructions(input):
    total = 0
    return total


def read_input():
    # return example
    with open("./input.txt") as file:
        return file.read()


def apply_beam(matrix, beam):
    for x, y in beam.history:
        matrix[y][x] = "#"
    return matrix


def main():
    matrix = process_input(read_input())
    total = 0
    beams = []
    beam = Beam(0, 0, "down", (len(matrix[0]), len(matrix)))  # cheating with the down!
    beams.append(beam)

    # create a matrix the same size as the input
    # fill it with "."
    tracker = [["." for _ in range(len(matrix[0]))] for _ in range(len(matrix))]

    cycles = 0
    while cycles < 1000:
        cycles += 1
        beams = [beam for beam in beams if not beam.ready_for_destruction]
        for beam in beams:
            apply_beam(tracker, beam)
            beam.move()
            if (
                beam.x < 0
                or beam.x > len(matrix[0])
                or beam.y < 0
                or beam.y > len(matrix)
            ):
                beams.remove(beam)
                continue
            if matrix[beam.y][beam.x] == "|":
                if beam.direction == "right":
                    beam.direction = "down"
                    beams.append(
                        Beam(beam.x, beam.y, "up", (len(matrix[0]), len(matrix)))
                    )
                elif beam.direction == "left":
                    beam.direction = "up"
                    beams.append(
                        Beam(beam.x, beam.y, "down", (len(matrix[0]), len(matrix)))
                    )
                elif beam.direction == "up":
                    beam.direction = "up"
                elif beam.direction == "down":
                    beam.direction = "down"
            elif matrix[beam.y][beam.x] == "-":
                if beam.direction == "right":
                    beam.direction = "right"
                elif beam.direction == "left":
                    beam.direction = "left"
                elif beam.direction == "up":
                    beam.direction = "left"
                    beams.append(
                        Beam(beam.x, beam.y, "right", (len(matrix[0]), len(matrix)))
                    )
                elif beam.direction == "down":
                    beam.direction = "right"
                    beams.append(
                        Beam(beam.x, beam.y, "left", (len(matrix[0]), len(matrix)))
                    )
            elif matrix[beam.y][beam.x] == "/":
                if beam.direction == "right":
                    beam.direction = "up"
                elif beam.direction == "left":
                    beam.direction = "down"
                elif beam.direction == "up":
                    beam.direction = "right"
                elif beam.direction == "down":
                    beam.direction = "left"
            elif matrix[beam.y][beam.x] == "\\":
                if beam.direction == "right":
                    beam.direction = "down"
                elif beam.direction == "left":
                    beam.direction = "up"
                elif beam.direction == "up":
                    beam.direction = "left"
                elif beam.direction == "down":
                    beam.direction = "right"
            else:
                pass
        print_matrix(tracker)
        print(f"Beams: {len(beams)}")
        print("\n")

    for line in tracker:
        for char in line:
            if char == "#":
                total += 1
    print(total)


if __name__ == "__main__":
    main()
