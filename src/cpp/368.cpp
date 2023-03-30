#include <algorithm>
#include <functional>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> largestDivisibleSubset(vector<int> &nums) {
        const int n = nums.size();
        vector<int> dp(n, 1);
        vector<int> prev(n, -1);
        sort(nums.begin(), nums.end(), std::greater<int>());

        for (int i = 1; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[j] % nums[i] == 0 && dp[j] + 1 > dp[i]) {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
        }
        int result_idx = 0;
        for (int i = 1; i < n; i++) {
            if (dp[i] > dp[result_idx]) {
                result_idx = i;
            }
        }

        vector<int> ret;
        while (result_idx != -1) {
            ret.push_back(nums[result_idx]);
            result_idx = prev[result_idx];
        }

        return ret;
    }
};

int main() {
    Solution s;

    return 0;
}