#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class BSTIterator {
    vector<TreeNode *> stack;

public:
    BSTIterator(TreeNode *root) {
        while (root != nullptr) {
            stack.push_back(root);
            root = root->left;
        }
    }

    int next() {
        auto node = stack.back();
        auto val = node->val;

        if (node->right != nullptr) {
            node = node->right;
            do {
                stack.push_back(node);
                node = node->left;
            } while (node != nullptr);
            return val;
        }

        while (true) {
            stack.pop_back();
            if (stack.empty()) break;
            node = stack.back();
            if (node->val > val) {
                break;
            }
        }
        
        return val;
    }

    bool hasNext() {
        return !stack.empty();
    }
};

int main() {
    return 0;
}
