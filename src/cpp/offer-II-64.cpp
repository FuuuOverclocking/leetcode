#include "default.h"

class MagicDictionary {
    unordered_map<size_t, vector<string>> map;

public:
    MagicDictionary() {}

    void buildDict(vector<string> dictionary) {
        for (auto &&word : dictionary) {
            map[word.size()].push_back(word);
        }
    }

    bool search(string searchWord) {
        auto &&words = map[searchWord.size()];
        for (auto &&word : words) {
            int diff = 0;
            for (int i = 0; i < word.size(); i++) {
                if (word[i] != searchWord[i]) {
                    diff++;
                    if (diff > 1) break;
                }
            }
            if (diff == 1) return true;
        }
        return false;
    }
};

int main() { return 0; }
