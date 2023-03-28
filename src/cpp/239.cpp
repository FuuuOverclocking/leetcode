#include <cstddef>
#include <deque>
#include <utility>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> maxSlidingWindow(vector<int> &nums, int k) {
        vector<int> ret;
        ret.reserve(nums.size() - k + 1);

        deque<pair<int, size_t>> q;

        for (size_t i = 0; i < k; i++) {
            const int val = nums[i];
            while (!q.empty() && q.back().first <= val) {
                q.pop_back();
            }
            q.emplace_back(val, i);
        }
        ret.push_back(q.front().first);

        for (size_t i = k; i < nums.size(); i++) {
            const int val = nums[i];
            while (!q.empty() && q.back().first <= val) {
                q.pop_back();
            }
            q.emplace_back(val, i);

            while (q.front().second <= i - k) {
                q.pop_front();
            }
            ret.push_back(q.front().first);
        }

        return ret;
    }
};

int main() {
    Solution s;
    vector<int> v {1, -1};
    s.maxSlidingWindow(v, 1);
    return 0;
}