#include "default.h"
#include <vector>

class Solution {
public:
    vector<vector<int>> combinationSum(const vector<int> &candidates, int target) {
        vector<vector<int>> ret;
        vector<int> combine;

        std::function<auto(int target, vector<int> &combine, int idx)->void> dfs;
        dfs = [&](int target, vector<int> &combine, int idx) -> void {
            if (idx == candidates.size()) {
                return;
            }
            if (target == 0) {
                ret.emplace_back(combine);
                return;
            }
            dfs(target, combine, idx + 1);
            if (target - candidates[idx] >= 0) {
                combine.emplace_back(candidates[idx]);
                dfs(target - candidates[idx], combine, idx);
                combine.pop_back();
            }
        };

        dfs(target, combine, 0);

        return ret;
    }
};

int main() {
    return 0;
}
