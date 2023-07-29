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
    unordered_map<int64_t, int> map;

    int dfs(TreeNode *node, int64_t sum, int targetSum) {
        if (node == nullptr) return 0;

        sum += node->val;
        int ret = 0;
        if (map.count(sum - targetSum)) {
            ret = map[sum - targetSum];
        }

        map[sum] += 1;
        ret += dfs(node->left, sum, targetSum);
        ret += dfs(node->right, sum, targetSum);
        map[sum] -= 1;

        return ret;
    }

public:
    int pathSum(TreeNode *root, int targetSum) {
        map[0] = 1;
        return dfs(root, 0, targetSum);
    }
};

int main() { return 0; }
