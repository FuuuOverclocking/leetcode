#include "default.h"

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

/*
节点按下面的定义二进制序列化：

Node := 0x0000                                      (if is nullptr)
      | 0x0001  val:int32_t  left:Node  right:Node  (else)
*/

class Codec {
    void _serialize(vector<int32_t> &vec, TreeNode *node) {
        if (node == nullptr) {
            vec.push_back(0);
        } else {
            vec.push_back(1);
            vec.push_back(node->val);

            _serialize(vec, node->left);
            _serialize(vec, node->right);
        }
    }

    TreeNode *_deserialize(vector<int32_t> &vec, size_t &pos) {
        if (vec[pos++] == 0) return nullptr;

        auto node = new TreeNode(vec[pos++]);
        node->left = _deserialize(vec, pos);
        node->right = _deserialize(vec, pos);
        return node;
    }

public:
    // Encodes a tree to a single string.
    string serialize(TreeNode *root) {
        vector<int32_t> vec;
        _serialize(vec, root);
        string str(reinterpret_cast<char *>(&*vec.begin()),
                   reinterpret_cast<char *>(&*vec.end()));
        return str;
    }

    // Decodes your encoded data to tree.
    TreeNode *deserialize(string str) {
        vector<int32_t> vec(reinterpret_cast<int32_t *>(&*str.begin()),
                            reinterpret_cast<int32_t *>(&*str.end()));
        size_t pos = 0;
        return _deserialize(vec, pos);
    }
};

int main() {
    auto n1 = new TreeNode(1);
    auto n2 = new TreeNode(2);
    auto n3 = new TreeNode(3);
    auto n4 = new TreeNode(4);
    auto n5 = new TreeNode(5);

    n1->left = n2;
    n1->right = n3;
    n3->left = n4;
    n3->right = n5;

    Codec c;
    auto s = c.serialize(n1);
    auto nn = c.deserialize(s);
    cout << nn->val << endl;
    return 0;
}
