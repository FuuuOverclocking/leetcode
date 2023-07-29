#include <algorithm>
#include <array>
#include <queue>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int largestPathValue(string colors, vector<vector<int>> &edges) {
        const size_t n = colors.size();

        // 拓扑排序，找入度为 0 的节点
        vector<vector<int>> adj_tbl(n);
        vector<int> in_degree(n);
        for (auto &edge : edges) {
            adj_tbl[edge[0]].push_back(edge[1]);
            in_degree[edge[1]]++;
        }

        queue<int> q;
        for (size_t i = 0; i < n; i++) {
            if (in_degree[i] == 0) {
                q.push(i);
            }
        }

        int found = 0;
        // dp(i, c) = 节点 i 的上游节点中，颜色 c 的最大值 + 1
        vector<array<int, 26>> dp(n);
        while (!q.empty()) {
            found++;
            int from = q.front();
            q.pop();
            char color = colors[from] - 'a';
            dp[from][color]++;
            for (auto to : adj_tbl[from]) {
                in_degree[to]--;
                if (in_degree[to] == 0) {
                    q.push(to);
                }
                for (char c = 0; c < 26; c++) {
                    dp[to][c] = max(dp[to][c], dp[from][c]);
                }
            }
        }
        if (found != n) {
            return -1;
        }
        int ans = 0;
        for (int i = 0; i < n; ++i) {
            ans = max(ans, *max_element(dp[i].begin(), dp[i].end()));
        }
        return ans;
    }
};

int main() { return 0; }