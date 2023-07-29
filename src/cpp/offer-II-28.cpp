#include "default.h"

class Node {
public:
    int val;
    Node *prev;
    Node *next;
    Node *child;
};

class Solution {
public:
    Node *flatten(Node *head) {
        if (head == nullptr) return nullptr;
        auto [new_head, new_tail]{_flatten(head)};
        return new_head;
    }

    pair<Node *, Node *> _flatten(Node *head) {
        Node *curr = head;

        while (true) {
            if (curr->child) {
                auto [child_head, child_tail]{_flatten(curr->child)};
                curr->child = nullptr;
                auto next = curr->next;

                curr->next = child_head;
                child_head->prev = curr;

                if (next != nullptr) {
                    child_tail->next = next;
                    next->prev = child_tail;
                }
            }
            if (curr->next != nullptr) {
                curr = curr->next;
            } else {
                break;
            }
        }

        return {head, curr};
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
