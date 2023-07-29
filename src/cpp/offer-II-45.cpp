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
public:
    int findBottomLeftValue(TreeNode *root) {
        auto [h, val] = dfs(root);
        return val;
    }
    pair<int, int> dfs(TreeNode *node) {
        if (node->left == nullptr && node->right == nullptr) {
            return {1, node->val};
        }
        if (node->left == nullptr) {
            auto [h, val] = dfs(node->right);
            return {h + 1, val};
        }
        if (node->right == nullptr) {
            auto [h, val] = dfs(node->left);
            return {h + 1, val};
        }
        auto [h_l, val_l] = dfs(node->left);
        auto [h_r, val_r] = dfs(node->right);
        if (h_l >= h_r) {
            return {h_l + 1, val_l};
        } else {
            return {h_r + 1, val_r};
        }
    }
};

int main() { return 0; }
