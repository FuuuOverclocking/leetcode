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
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        stack<int> s1;
        stack<int> s2;

        while (l1 != nullptr) {
            s1.push(l1->val);
            l1 = l1->next;
        }
        while (l2 != nullptr) {
            s2.push(l2->val);
            l2 = l2->next;
        }
        ListNode* curr = nullptr;
        auto carry = 0;
        while (!(s1.empty() && s2.empty() && carry == 0)) {
            int a = 0;
            int b = 0;
            if (!s1.empty()) {
                a = s1.top();
                s1.pop();
            }
            if (!s2.empty()) {
                b = s2.top();
                s2.pop();
            }
            int sum = a + b + carry;
            carry = sum / 10;
            curr = new ListNode(sum % 10, curr);
        }
        return curr;
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
