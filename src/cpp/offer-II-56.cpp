#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

class Solution {
    unordered_set<int> map;

public:
    bool findTarget(TreeNode *root, int k) {
        if (root == nullptr) return false;
        if (map.count(k - root->val)) return true;

        map.insert(root->val);
        return findTarget(root->left, k) || findTarget(root->right, k);
    }
};

int main() { return 0; }
