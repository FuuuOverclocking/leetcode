#include "default.h"

class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size() || s == t){
            return false;
        }
        int count[26]{0};
        for (auto ch : s) {
            count[ch - 'a']++;
        }
        for (auto ch : t) {
            count[ch - 'a']--;
            if (count[ch - 'a'] < 0) {
                return false;
            }
        }
        return true;
    }
};

int main() {
    Solution solu;
    solu.isAnagram("rat", "car");
    return 0;
}
