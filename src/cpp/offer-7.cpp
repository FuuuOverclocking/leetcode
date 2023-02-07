#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
public:
    TreeNode *buildTree(vector<int> &preorder, vector<int> &inorder) {
        unordered_map<int, int> inorder_map;
        for (int i = 0; i < inorder.size(); i++) {
            inorder_map[inorder[i]] = i;
        }

        return buildTreeHelper(preorder, 0, preorder.size() - 1,
                               inorder, 0, inorder.size() - 1, inorder_map);
    }

private:
    TreeNode *buildTreeHelper(vector<int> &preorder, int p_start, int p_end,
                              vector<int> &inorder, int i_start, int i_end, unordered_map<int, int> &inorder_map) {
        if (p_start > p_end || i_start > i_end) {
            return NULL;
        }

        TreeNode *root = new TreeNode(preorder[p_start]);
        int i_root = inorder_map[preorder[p_start]];
        int left_tree_size = i_root - i_start;

        root->left = buildTreeHelper(preorder, p_start + 1, p_start + left_tree_size,
                                     inorder, i_start, i_root - 1, inorder_map);

        root->right = buildTreeHelper(preorder, p_start + left_tree_size + 1, p_end,
                                      inorder, i_root + 1, i_end, inorder_map);

        return root;
    }
};

int main() { return 0; }
