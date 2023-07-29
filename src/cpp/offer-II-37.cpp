#include "default.h"

class Solution {
public:
    vector<int> asteroidCollision(vector<int> &asteroids) {
        vector<int> s;
        for (auto num : asteroids) {
            bool reserve = true;
            while (true) {
                if (num > 0 || s.empty() || s.back() < 0) {
                    break;
                }
                auto top = s.back();
                if (top > -num) {
                    reserve = false;
                    break;
                }
                if (top == -num) {
                    reserve = false;
                    s.pop_back();
                    break;
                }
                s.pop_back();
            }
            if (reserve) s.push_back(num);
        }
        return s;
    }
};

int main() {
    Solution solu;
    return 0;
}
