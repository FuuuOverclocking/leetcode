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
    ListNode *sortList(ListNode *head) {
        if (head == nullptr) return nullptr;
        if (head->next == nullptr) return head;

        auto mid_node = findMid(head);

        auto list_1 = head;
        auto list_2 = mid_node->next;

        mid_node->next = nullptr;

        list_1 = sortList(list_1);
        list_2 = sortList(list_2);

        if (list_1->val < list_2->val) {
            head = list_1;
            list_1 = list_1->next;
        } else {
            head = list_2;
            list_2 = list_2->next;
        }

        auto tail = head;

        while (true) {
            if (list_1 == nullptr) {
                tail->next = list_2;
                break;
            }
            if (list_2 == nullptr) {
                tail->next = list_1;
                break;
            }
            if (list_1->val < list_2->val) {
                tail->next = list_1;
                tail = tail->next;
                list_1 = list_1->next;
            } else {
                tail->next = list_2;
                tail = tail->next;
                list_2 = list_2->next;
            }
        }
        return head;
    }

    auto findMid(ListNode *head) -> ListNode * {
        auto slow = head;
        auto fast = head;

        while (fast->next != nullptr && fast->next->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        return slow;
    }
};

int main() {
    auto n1 = new ListNode(3);
    auto n2 = new ListNode(4);
    auto n3 = new ListNode(1);
    n1->next = n2;
    n2->next = n3;
    Solution s;
    s.sortList(n1);
    return 0;
}
