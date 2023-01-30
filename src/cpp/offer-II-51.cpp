#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
    int dfs(TreeNode *node, int &max_sum) {
        int ret = node->val;
        int left = 0;
        int right = 0;

        max_sum = max(max_sum, node->val);
        if (node->left != nullptr) {
            left = dfs(node->left, max_sum);
            ret = max(ret, left + node->val);
            max_sum = max(max_sum, left + node->val);
        }
        if (node->right != nullptr) {
            right = dfs(node->right, max_sum);
            ret = max(ret, right + node->val);
            max_sum = max(max_sum, right + node->val);
        }
        if (node->left != nullptr && node->right != nullptr) {
            max_sum = max(max_sum, left + right + node->val);
        }

        return ret;
    }

public:
    int maxPathSum(TreeNode *root) {
        int max_sum = INT_MIN;
        dfs(root, max_sum);
        return max_sum;
    }
};

int main() {
    return 0;
}
