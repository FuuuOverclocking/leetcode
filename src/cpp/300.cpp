#include <algorithm>
#include <cstddef>
#include <vector>

using namespace std;

class Solution {
public:
    int lengthOfLIS(vector<int> &nums) {
        int ret = 0;
        // tails[i] = a, 则目前为止, 存在长度为 i 的严格单增子序列,
        // 最后一位数字是 a
        vector<int> tails(nums.size() + 1, 0);

        for (auto num : nums) {
            // 二分查找 tails[1..=ret], 找首个 tails[i] >= num,
            // 若找到, 令 tails[i] = num, 否则 ret++, tails[ret] = num
            if (ret < 1) {
                ret++;
                tails[ret] = num;
                continue;
            }
            int l = 1;
            int r = ret;
            int mid;
            while (l <= r) {
                mid = l + (r - l) / 2;
                if (tails[mid] < num) {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            tails[l] = num;
            if (l > ret) {
                ret = l;
            }
        }

        return ret;
    }
};

int main() { return 0; }
