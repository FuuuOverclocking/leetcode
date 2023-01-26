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
public:
    vector<int> largestValues(TreeNode *root) {
        if (root == nullptr) return {};

        vector<int> ret;
        queue<pair<int, TreeNode *>> q;
        q.push({1, root});

        int level_max = root->val;
        int last_h = 1;
        while (!q.empty()) {
            auto [h, node] = q.front();
            q.pop();
            if (node->left != nullptr) q.push({h + 1, node->left});
            if (node->right != nullptr) q.push({h + 1, node->right});

            if (last_h != h) {
                ret.push_back(level_max);
                level_max = node->val;
                last_h = h;
            } else {
                level_max = max(level_max, node->val);
            }
        }
        ret.push_back(level_max);

        return ret;
    }
};

int main() {
    return 0;
}
