#include "default.h"

class RandomizedSet {
    vector<int> arr;
    unordered_map<int, int> map;

public:
    /** Initialize your data structure here. */
    RandomizedSet() {}

    /** Inserts a value to the set. Returns true if the set did not already
     * contain the specified element. */
    bool insert(int val) {
        if (map.count(val) != 0) {
            return false;
        }
        arr.push_back(val);
        map.insert({val, arr.size() - 1});
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the
     * specified element. */
    bool remove(int val) {
        if (map.count(val) == 0) {
            return false;
        }
        int idx = map[val];
        arr[idx] = arr.back();
        map[arr.back()] = idx;
        arr.pop_back();
        map.erase(val);
        return true;
    }

    /** Get a random element from the set. */
    int getRandom() {
        int idx = rand() % arr.size();
        return arr[idx];
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */

int main() {
    // Solution solu;
    // solu
    return 0;
}
