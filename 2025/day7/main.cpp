#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

vector<vector<string>> get_input(const string& file_path) {
    ifstream f(file_path);
    vector<vector<string>> result;
    string line;
    
    while (getline(f, line)) {
        vector<string> temp;
        for (char c : line) {
            temp.push_back(string(1, c));
        }
        result.push_back(temp);
    }
    
    return result;
}

long long dfs(const vector<vector<string>>& grid, int i, int j, 
        unordered_map<long long, long long>& memo) {
    if (i >= static_cast<int>(grid.size())) {
        return 1;
    }
    if (i < 0 || j < 0 || j >= static_cast<int>(grid[0].size())) {
        return 0;
    }
    
    // Memoization: Check if we've already computed the result for this (i, j) position
    // This prevents redundant calculations when multiple paths converge at the same position
    // Without this, the algorithm has exponential time complexity O(2^n) where n = number of obstacles
    // With memoization, it becomes O(rows * cols) since each position is computed at most once
    long long key = (static_cast<long long>(i) << 32) | static_cast<long long>(j);
    if (memo.find(key) != memo.end()) {
        return memo[key];
    }
    
    long long result;
    if (grid[i][j] == "." || grid[i][j] == "S") {
        // Empty space: continue straight down
        result = dfs(grid, i + 1, j, memo);
    } else if (grid[i][j] == "^") {
        // Obstacle (splitter): branch into two paths (left and right)
        // Many of these branches can reconverge at the same positions later
        // Memoization ensures we don't recalculate paths from those positions
        result = dfs(grid, i + 1, j - 1, memo) + dfs(grid, i + 1, j + 1, memo);
    } else {
        // Unknown character: treat as empty space
        result = dfs(grid, i + 1, j, memo);
    }
    
    // Store the computed result so future calls with the same (i, j) can reuse it
    memo[key] = result;
    return result;
}

void part2() {
    auto input_content = get_input("./input.txt");
    long long total = 0;
    
    for (size_t i = 0; i < input_content.size(); i++) {
        for (size_t j = 0; j < input_content[i].size(); j++) {
            if (input_content[i][j] == "S") {
                // Initialize memoization map to cache results for each (i, j) position
                unordered_map<long long, long long> memo;
                total = dfs(input_content, i + 1, j, memo);
                break;
            }
        }
        if (total > 0) break;
    }
    
    cout << total << endl;
}

int main() {
    part2();
    return 0;
}