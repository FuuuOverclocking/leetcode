#include "default.h"

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
    pair<ListNode *, ListNode *> reverseList(ListNode *head) {
        if (head == nullptr) {
            return {nullptr, nullptr};
        }
        if (head->next == nullptr) {
            return {head, head};
        }
        auto [new_head, new_tail]{reverseList(head->next)};
        new_tail->next = head;
        head->next = nullptr;
        return {new_head, head};
    }

public:
    ListNode *reverseList(ListNode *head) {
        auto [new_head, new_tail]{reverseList(head)};
        return new_head;
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
