#include "default.h"

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode *removeNthFromEnd(ListNode *head, int n) {
        ListNode *pr = head;

        ListNode tmp(0, head);
        ListNode *pl = head = &tmp;

        n -= 1;
        while (true) {
            if (pr->next == nullptr) {
                pl->next = pl->next->next;
                return head->next;
            }
            pr = pr->next;
            if (n == 0) {
                pl = pl->next;
            } else {
                n -= 1;
            }
        }
    }
};

int main() { return 0; }
