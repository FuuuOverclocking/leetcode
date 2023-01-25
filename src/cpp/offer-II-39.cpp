#include "default.h"

class Solution {
    struct Elem {
        int idx;
        int val;
        int l;
    };

public:
    int largestRectangleArea(vector<int> &heights) {
        int ret = 0;
        vector<Elem> s;
        s.reserve(heights.size());

        for (int idx = 0; idx < heights.size(); idx++) {
            int val = heights[idx];
            while (!s.empty() && s.back().val >= val) {
                auto el = s.back();
                s.pop_back();
                ret = max(ret, el.val * (idx - el.l));
            }
            int l = s.empty() ? 0 : s.back().idx + 1;
            s.push_back({idx, val, l});
        }
        int idx = heights.size();
        while (!s.empty()) {
            auto el = s.back();
            s.pop_back();
            ret = max(ret, el.val * (idx - el.l));
        }

        return ret;
    }
};

int main() {
    Solution solu;
    vector<int> arr = {2, 1, 5, 6, 2, 3};
    solu.largestRectangleArea(arr);
    return 0;
}
