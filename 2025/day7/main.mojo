

def get_input(file_path: String) -> List[List[String]]:
    var f = open(file_path, "r")
    var result = List[List[String]]()
    for i in f.read().splitlines():
        temp = List[String]()
        for c in i.split(""):
            temp.append(String(c))
        result.append(temp^)
    return result^

def part1():
    var input_content = get_input("./input.txt")
    var total = 0
    for i, line in enumerate(input_content):
        for j, val in enumerate(line):
            if i + 1 >= len(input_content):
                break

            if (val == "S" or val == "|"):
                if input_content[i + 1][j] == "^":
                    if j + 1 < len(line):
                        input_content[i + 1][j + 1] = "|"
                    if j - 1 >= 0:
                        input_content[i + 1][j - 1] = "|"
                    total += 1
                if input_content[i + 1][j] == ".":
                    input_content[i + 1][j] = "|"
                    
    print(total)

fn dfs(borrowed grid: List[List[String]], i: Int, j: Int, inout memo: Dict[String, Int]) -> Int:
    if i >= len(grid):
        return 1
    if i < 0 or j < 0 or j >= len(grid[0]):
        return 0

    # Memoization: Check if we've already computed the result for this (i, j) position
    # This prevents redundant calculations when multiple paths converge at the same position
    # Without this, the algorithm has exponential time complexity O(2^n) where n = number of obstacles
    # With memoization, it becomes O(rows * cols) since each position is computed at most once
    var key = String(i) + "," + String(j)
    if key in memo:
        return memo[key]

    var result: Int
    if (grid[i][j] == "." or grid[i][j] == "S"):
        result = dfs(grid, i + 1, j, memo)
    else:
        # When hitting an obstacle, we branch into two paths (left and right)
        # Many of these branches can reconverge at the same positions later
        # Memoization ensures we don't recalculate paths from those positions
        result = dfs(grid, i + 1, j - 1, memo) + dfs(grid, i + 1, j + 1, memo)
    
    # Store the computed result so future calls with the same (i, j) can reuse it
    memo[key] = result
    return result

def part2():
    var input_content = get_input("./input.txt")
    var total = 0

    for i, line in enumerate(input_content):
        for j, val in enumerate(line):
            if i + 1 >= len(input_content):
                break

            if (val == "S"):
                # Initialize memoization dictionary to cache results for each (i, j) position
                var memo = Dict[String, Int]()
                total = dfs(input_content, i + 1, j, memo)
                break
                    
    print(total)
    
    

def main():
    # part1()
    part2()