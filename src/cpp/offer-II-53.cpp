#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

class Solution {
    /**
     * @brief 若不能在子树找到后继，返回 nullptr, 否则返回后继
     *
     * @param node 非空
     * @param search_val
     * @return TreeNode*
     */
    auto search(TreeNode *node, int search_val) -> TreeNode * {
        if (node->val <= search_val) {
            if (node->right == nullptr) return nullptr;
            return search(node->right, search_val);
        }
        if (node->left == nullptr) {
            return node;
        }
        auto n = search(node->left, search_val);
        return n == nullptr ? node : n;
    }

public:
    TreeNode *inorderSuccessor(TreeNode *root, TreeNode *p) {
        if (p->right != nullptr) {
            p = p->right;
            while (true) {
                if (p->left != nullptr) {
                    p = p->left;
                    continue;
                }
                return p;
            }
        }
        return search(root, p->val);
    }
};

int main() {
    return 0;
}
