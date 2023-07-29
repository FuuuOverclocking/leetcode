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
    auto dfs(TreeNode *node, int &lastSum) -> void {
        if (node->right != nullptr) {
            dfs(node->right, lastSum);
        }
        node->val += lastSum;
        lastSum = node->val;
        if (node->left != nullptr) {
            dfs(node->left, lastSum);
        }
    }

public:
    TreeNode *convertBST(TreeNode *root) {
        if (root == nullptr) return nullptr;
        int lastSum = 0;
        dfs(root, lastSum);
        return root;
    }
};

int main() { return 0; }
