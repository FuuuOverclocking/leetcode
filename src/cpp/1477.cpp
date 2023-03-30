#include <algorithm>
#include <climits>
#include <functional>
#include <iostream>
#include <utility>
#include <vector>

using namespace std;

class Solution {
    struct Range {
        int l;
        int r;
    };

public:
    int minSumOfLengths(vector<int> &arr, int target) {
        Range range{0, 0};
        int sum = 0;
        auto get_next = [&]() -> pair<Range, bool> {
            int &l = range.l;
            int &r = range.r;
            while (l <= r && r <= arr.size()) {
                if (sum == target) {
                    if (r != arr.size()) {
                        sum -= arr[l];
                        sum += arr[r];
                    }
                    l++;
                    r++;
                    return {{l - 1, r - 1}, true};
                }
                if (sum < target) {
                    if (r == arr.size()) {
                        r++;
                        break;
                    }
                    sum += arr[r];
                    r++;
                } else {
                    sum -= arr[l];
                    l++;
                }
            }
            return {{0, 0}, false};
        };

        int ret = INT_MAX;
        // [r] = v, [0..r] 中子数组的最短长度，0 表示还没求值，INT_MAX 表示不存在
        // dp[r] = min(dp[r-1], len)
        vector<int> dp(arr.size() + 1, 0);

        std::function<auto(int r, int len)->void> update_dp;
        update_dp = [&](int r, int len) {
            if (dp[r] != 0) return;
            if (r == 0) {
                dp[r] = len == -1 ? INT_MAX : len;
                return;
            }
            update_dp(r - 1, -1);
            dp[r] = len == -1 ? dp[r - 1] : min(dp[r - 1], len);
        };

        decltype(get_next()) iter;
        while (iter = get_next(), iter.second) {
            auto rng = iter.first;
            auto l = rng.l, r = rng.r;
            auto len = r - l;

            update_dp(r, len);
            if (dp[l] != INT_MAX) {
                ret = min(ret, dp[l] + len);
            }
        }

        return ret == INT_MAX ? -1 : ret;
    }
};

int main() {
    Solution s;
    vector<int> arr{3, 2, 2, 4, 3};
    cout << s.minSumOfLengths(arr, 3) << endl;
    return 0;
}