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
    int _sumNumbers(TreeNode *node, int num) {
        num = num * 10 + node->val;
        if (node->left == nullptr && node->right == nullptr) {
            return num;
        }

        int left = node->left == nullptr ? 0 : _sumNumbers(node->left, num);
        int right = node->right == nullptr ? 0 : _sumNumbers(node->right, num);
        return left + right;
    }

public:
    int sumNumbers(TreeNode *root) { return _sumNumbers(root, 0); }
};

int main() { return 0; }
