#include "default.h"

class Node {
public:
    int val;
    Node *next;

    Node() {}

    Node(int _val) {
        val = _val;
        next = NULL;
    }

    Node(int _val, Node *_next) {
        val = _val;
        next = _next;
    }
};

class Solution {
public:
    Node *insert(Node *head, int insertVal) {
        auto node = new Node(insertVal);

        // 0 个节点
        if (head == nullptr) {
            node->next = node;
            return node;
        }

        // 1 个节点
        if (head->next == head) {
            head->next = node;
            node->next = head;
            return head;
        }

        // 转一圈, 能找到 [min, max]
        // 以及 min != max 时, node_min, node_max
        // min == max 时, 任意插, 不妨插在 head 后；否则,
        // insertVal 不在 (min, max) 时, 插在 node_max, node_min 中间
        // 否则, 存在小于 insertVal 的最**的那个节点, 插在那个节点后面

        auto curr = head;
        Node *node_min = nullptr;
        Node *node_max = nullptr;

        Node *p = nullptr;
        bool sure = false;

        do {
            if (curr->val > curr->next->val) {
                node_max = curr;
                node_min = curr->next;
            }
            if (curr->val < insertVal && !sure) {
                p = curr;
                if (curr->next->val >= insertVal) {
                    sure = true;
                }
            }
            curr = curr->next;
        } while (curr != head);

        if (node_min == nullptr) {
            node->next = head->next;
            head->next = node;
        } else {
            if (insertVal <= node_min->val || insertVal >= node_max->val) {
                node->next = node_min;
                node_max->next = node;
            } else {
                node->next = p->next;
                p->next = node;
            }
        }

        return head;
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
