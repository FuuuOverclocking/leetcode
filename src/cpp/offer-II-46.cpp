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
    vector<int> rightSideView(TreeNode *root) {
        vector<int> ret;
        if (root == nullptr) return ret;
        dfs(ret, root, 0);
        return ret;
    }
    void dfs(vector<int> &ret, TreeNode *node, int idx) {
        if (idx >= ret.size()) {
            ret.push_back(node->val);
        } else {
            ret[idx] = node->val;
        }
        if (node->left != nullptr) dfs(ret, node->left, idx + 1);
        if (node->right != nullptr) dfs(ret, node->right, idx + 1);
    }
};

int main() { return 0; }
