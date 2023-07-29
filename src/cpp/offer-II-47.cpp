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
    TreeNode *pruneTree(TreeNode *root) {
        if (is_zero_subtree(root)) return nullptr;

        prune(root);
        return root;
    }

    // 完成剪枝, 其中 node 本身不用剪
    void prune(TreeNode *node) {
        if (node->left != nullptr) {
            if (is_zero_subtree(node->left)) {
                node->left = nullptr;
            } else {
                prune(node->left);
            }
        }
        if (node->right != nullptr) {
            if (is_zero_subtree(node->right)) {
                node->right = nullptr;
            } else {
                prune(node->right);
            }
        }
    }

    // 如果 node 及以下全是 0, 返回 true, 否则 false
    bool is_zero_subtree(TreeNode *node) {
        if (node->val == 1) return false;

        bool left = true;
        bool right = true;
        if (node->left != nullptr) {
            left = is_zero_subtree(node->left);
        }
        if (node->right != nullptr) {
            right = is_zero_subtree(node->right);
        }
        return left && right;
    }
};

int main() { return 0; }
