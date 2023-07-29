#include "default.h"
#include <algorithm>
#include <vector>

using namespace std;

const u64 N = 1000000007;

class Solution {
public:
    int countVowelPermutation(int n) {
        u64 storage[10] = {1, 1, 1, 1, 1};
        u64 *dp = storage;
        u64 *dp_next = dp + 5;

        for (int i = 2; i <= n; i++) {
            dp_next[0] = dp[1];

            dp_next[1] = dp[0];
            dp_next[1] += dp[2];
            dp_next[1] %= N;

            dp_next[2] = dp[0];
            dp_next[2] += dp[1];
            dp_next[2] %= N;
            dp_next[2] += dp[3];
            dp_next[1] %= N;
            dp_next[2] += dp[4];
            dp_next[1] %= N;

            dp_next[3] = dp[2];
            dp_next[3] += dp[4];
            dp_next[3] %= N;

            dp_next[4] = dp[0];

            swap(dp, dp_next);
        }
        u64 ret = 0;
        ret += dp[0];
        ret %= N;
        ret += dp[1];
        ret %= N;
        ret += dp[2];
        ret %= N;
        ret += dp[3];
        ret %= N;
        ret += dp[4];
        ret %= N;
        return ret;
    }
};

int main() { return 0; }