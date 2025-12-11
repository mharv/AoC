#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <unordered_map>
#include <iomanip>

using namespace std;

// Statistics counters
int total_calls = 0;
int cache_hits = 0;
int cache_misses = 0;

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
        unordered_map<long long, long long>& memo, int depth = 0) {
    total_calls++;
    
    // Visualize the call with indentation based on depth
    string indent(depth * 2, ' ');
    cout << indent << "dfs(" << i << ", " << j << ") ";
    
    if (i >= static_cast<int>(grid.size())) {
        cout << "-> bottom reached = 1" << endl;
        return 1;
    }
    if (i < 0 || j < 0 || j >= static_cast<int>(grid[0].size())) {
        cout << "-> out of bounds = 0" << endl;
        return 0;
    }
    
    long long key = (static_cast<long long>(i) << 32) | static_cast<long long>(j);
    if (memo.find(key) != memo.end()) {
        cache_hits++;
        cout << "-> CACHE HIT! returning " << memo[key] << endl;
        return memo[key];
    }
    
    cache_misses++;
    cout << "at '" << grid[i][j] << "'" << endl;
    
    long long result;
    if (grid[i][j] == "." || grid[i][j] == "S") {
        result = dfs(grid, i + 1, j, memo, depth + 1);
    } else if (grid[i][j] == "^") {
        cout << indent << "  SPLIT: going left and right" << endl;
        long long left = dfs(grid, i + 1, j - 1, memo, depth + 1);
        long long right = dfs(grid, i + 1, j + 1, memo, depth + 1);
        result = left + right;
        cout << indent << "  MERGE: " << left << " + " << right << " = " << result << endl;
    } else {
        result = dfs(grid, i + 1, j, memo, depth + 1);
    }
    
    memo[key] = result;
    cout << indent << "  -> cached[" << i << "," << j << "] = " << result << endl;
    return result;
}

void part2() {
    auto input_content = get_input("./input.txt");
    long long total = 0;
    
    // Print the grid
    cout << "Grid:" << endl;
    for (size_t i = 0; i < input_content.size(); i++) {
        for (size_t j = 0; j < input_content[i].size(); j++) {
            cout << input_content[i][j];
        }
        cout << endl;
    }
    cout << endl;
    
    for (size_t i = 0; i < input_content.size(); i++) {
        for (size_t j = 0; j < input_content[i].size(); j++) {
            if (input_content[i][j] == "S") {
                unordered_map<long long, long long> memo;
                cout << "Starting from S at position (" << i << ", " << j << ")" << endl;
                cout << "================================================" << endl;
                total = dfs(input_content, i + 1, j, memo);
                cout << "================================================" << endl;
                cout << "\nMemoization Statistics:" << endl;
                cout << "  Total function calls: " << total_calls << endl;
                cout << "  Cache hits: " << cache_hits << " (" 
                     << fixed << setprecision(1) 
                     << (100.0 * cache_hits / total_calls) << "%)" << endl;
                cout << "  Cache misses: " << cache_misses << " (" 
                     << fixed << setprecision(1) 
                     << (100.0 * cache_misses / total_calls) << "%)" << endl;
                cout << "  Cached positions: " << memo.size() << endl;
                cout << "\nWithout memoization, this would require approximately " 
                     << cache_hits + cache_misses << " + " << cache_hits 
                     << " = " << total_calls + cache_hits 
                     << " function calls (or more due to recomputation)" << endl;
                break;
            }
        }
        if (total > 0) break;
    }
    
    cout << "\nFinal answer: " << total << " timelines" << endl;
}

int main() {
    part2();
    return 0;
}
