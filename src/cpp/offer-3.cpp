#include "default.h"

class Solution {
public:
    int findRepeatNumber(vector<int> &nums) {
        bool record[100000] = {0};
        for (auto n : nums) {
            if (record[n]) {
                return n;
            }
            record[n] = true;
        }
        return -1;
    }
};

int main() { return 0; }
