#include "default.h"
#include <queue>

class Solution {
public:
    int miceAndCheese(vector<int> &reward1, vector<int> &reward2, int k) {
        const int n = reward1.size();
        auto comparer = [&](int l, int r) {
            auto l_val = reward1[l] - reward2[l];
            auto r_val = reward1[r] - reward2[r];
            return l_val > r_val; // 最小堆
        };
        priority_queue<int, vector<int>, decltype(comparer)> q(comparer);
        if (k != 0) {
            for (int i = 0; i < n; i++) {
                if (q.size() != k) {
                    q.push(i);
                    continue;
                }
                if (comparer(i, q.top())) {
                    q.pop();
                    q.push(i);
                }
            }
        }
        int score = 0;
        for (auto num : reward2) {
            score += num;
        }
        while (!q.empty()) {
            auto idx = q.top();
            q.pop();
            score += reward1[idx] - reward2[idx];
        }
        return score;
    }
};

int main() {
    Solution s;
    vector<int> reward1{1, 2, 1, 2, 1, 2};
    vector<int> reward2{2, 1, 1, 2, 2, 1};
    s.miceAndCheese(reward1, reward2, 0);
    return 0;
}
