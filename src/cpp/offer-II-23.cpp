#include "default.h"

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        auto p1 = headA;
        auto p2 = headB;
        auto foundNull = false;

        while (true) {
            if (p1 == p2) {
                return p1;
            }

            if (p1->next == nullptr) {
                if (foundNull) {
                    return nullptr;
                }
                foundNull = true;
                p1 = headB;
            } else {
                p1 = p1->next;
            }
            if (p2->next == nullptr) {
                p2 = headA;
            } else {
                p2 = p2->next;
            }
        }
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
