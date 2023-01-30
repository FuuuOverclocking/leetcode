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
    pair<TreeNode *, TreeNode *> _increasingBST(TreeNode *root) {
        TreeNode *head;
        TreeNode *tail;
        if (root->left != nullptr) {
            auto [_head, _tail] = _increasingBST(root->left);
            head = _head;
            _tail->right = root;
            root->left = nullptr;
        } else {
            head = root;
        }
        if (root->right != nullptr) {
            auto [_head, _tail] = _increasingBST(root->right);
            tail = _tail;
            root->right = _head;
        } else {
            tail = root;
        }
        
        return {head, tail};
    }

public:
    TreeNode *increasingBST(TreeNode *root) {
        return _increasingBST(root).first;
    }
};

int main() {
    return 0;
}
