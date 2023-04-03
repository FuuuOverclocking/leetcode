#include <string>
#include <string_view>
#include <vector>
using namespace std;

class Solution {
public:
    bool isInterleave(string s1, string s2, string target) {
        const int n = s1.size();
        const int m = s2.size();
        const int t = target.size();

        if (n + m != t) return false;

        // dp[i,j]: bool := 能够由 s1[0..i] 和 s2[0..j] 交错构成 target[0..i+j]
        // dp[i,j] = d[i-1,j] && s1[i-1] == target[i+j-1]
        //        || d[i,j-1] && s2[j-1] == target[i+j-1]
        // 滚动更新
        vector<bool> dp(n + m + 1, false);
        dp[0] = true;

        for (int i = 0; i <= n; i++) {
            for (int j = 0; j <= m; j++) {
                if (i != 0) {
                    dp[j] = dp[j] && s1[i - 1] == target[i + j - 1];
                }
                if (j != 0) {
                    dp[j] = dp[j] || (dp[j - 1] && s2[j - 1] == target[i + j - 1]);
                }
            }
        }
        return dp[m];
    }
};

int main() {
    return 0;
}
