#include "default.h"

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode *mergeKLists(vector<ListNode *> &lists) {
        size_t len = lists.size();
        if (len == 0) return nullptr;

        while (len != 1) {
            for (size_t i = 0; i < len; i += 2) {
                if (i + 1 < len) {
                    lists[i / 2] = merge2Lists(lists[i], lists[i + 1]);
                } else {
                    lists[i / 2] = lists[i];
                }
            }
            len = (len + 1) / 2;
        }
        return lists[0];
    }

    ListNode *merge2Lists(ListNode *l1, ListNode *l2) {
        ListNode sentinel;
        auto p = &sentinel;

        while (l1 != nullptr && l2 != nullptr) {
            if (l1->val < l2->val) {
                p->next = l1;
                l1 = l1->next;
            } else {
                p->next = l2;
                l2 = l2->next;
            }
            p = p->next;
        }
        if (l1 != nullptr) {
            p->next = l1;
        }
        if (l2 != nullptr) {
            p->next = l2;
        }
        return sentinel.next;
    }
};

int main() { return 0; }
