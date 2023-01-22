#include "default.h"

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        if (head == nullptr) {
            return nullptr;
        }
        ListNode *ps = head;
        ListNode *pf = head;
        int32_t t = 0;

        while (true) {
            ps = ps->next;
            if (ps == nullptr) {
                return nullptr;
            }
            pf = pf->next;
            if (pf == nullptr) {
                return nullptr;
            }
            pf = pf->next;
            if (pf == nullptr) {
                return nullptr;
            }
            t += 1;
            if (ps == pf) {
                break;
            }
        }
        ps = pf = head;
        while (true) {
            pf = pf->next;
            if (t == 0) {
                ps = ps->next;

            } else {
                t -= 1;
            }
            if (pf == ps) {
                return pf;
            }
        }
    }
};

int main() {
    Solution solu;
    solu.detectCycle(nullptr);
    return 0;
}
