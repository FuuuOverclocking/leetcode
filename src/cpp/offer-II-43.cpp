#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class CBTInserter {
    TreeNode *const root;
    TreeNode **last_full_level;
    size_t last_full_level_len;
    size_t curr;

    void fill(TreeNode **last_full_level,
              TreeNode *node,
              size_t curr_level_idx,
              size_t target_h,
              size_t curr_h) {
        if (target_h == curr_h) {
            last_full_level[curr_level_idx] = node;
            return;
        }
        fill(last_full_level,
             node->left,
             curr_level_idx * 2,
             target_h,
             curr_h + 1);
        fill(last_full_level,
             node->right,
             curr_level_idx * 2 + 1,
             target_h,
             curr_h + 1);
    }

public:
    CBTInserter(TreeNode *root) : root(root) {
        size_t h = 0;
        while (root != nullptr) {
            h += 1;
            root = root->right;
        }
        last_full_level_len = 1 << (h - 1);
        last_full_level = new TreeNode *[last_full_level_len];
        fill(last_full_level, this->root, 0, h, 1);

        for (size_t i = 0; i < last_full_level_len; i++) {
            if (last_full_level[i]->left == nullptr ||
                last_full_level[i]->right == nullptr) {
                curr = i;
                break;
            }
        }
    }

    int insert(int v) {
        auto node = last_full_level[curr];
        if (node->left == nullptr) {
            node->left = new TreeNode(v);
            return node->val;
        }
        node->right = new TreeNode(v);
        curr += 1;
        if (curr != last_full_level_len) {
            return node->val;
        }
        auto new_last_full_level_len = last_full_level_len * 2;
        auto new_last_full_level = new TreeNode *[new_last_full_level_len];
        for (size_t i = 0; i < last_full_level_len; i++) {
            new_last_full_level[i * 2] = last_full_level[i]->left;
            new_last_full_level[i * 2 + 1] = last_full_level[i]->right;
        }
        delete [] last_full_level;
        last_full_level = new_last_full_level;
        last_full_level_len = new_last_full_level_len;
        curr = 0;
        return node->val;
    }

    TreeNode *get_root() {
        return root;
    }
};

/**
 * Your CBTInserter object will be instantiated and called as such:
 * CBTInserter* obj = new CBTInserter(root);
 * int param_1 = obj->insert(v);
 * TreeNode* param_2 = obj->get_root();
 */

int main() {
    auto n1 = new TreeNode(1);
    // auto n2 = new TreeNode(2);
    // auto n3 = new TreeNode(3);
    // auto n4 = new TreeNode(4);
    // auto n5 = new TreeNode(5);
    // auto n6 = new TreeNode(6);

    // n1->left = n2;
    // n1->right = n3;
    // n2->left = n4;
    // n2->right = n5;
    // n3->left = n6;
    CBTInserter cbt(n1);
    cbt.insert(2);
    cbt.insert(3);
    return 0;
}
