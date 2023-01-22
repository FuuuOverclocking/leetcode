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
    // 1. 找到链表中间节点
    // 2. 反转后半链表
    // 3. 对比
    bool isPalindrome(ListNode *head) {
        auto center = getCenter(head);
        auto [new_head, new_tail]{reverseList(center->next)};
        center->next = nullptr;
        while (head != nullptr && new_head != nullptr) {
            if (head->val != new_head->val) {
                return false;
            }
            head = head->next;
            new_head = new_head->next;
        }
        return true;
    }

    // 链表不为空. 返回中间节点(奇数个), 或中间靠左(偶数个).
    ListNode *getCenter(ListNode *head) {
        ListNode *p1 = head;
        ListNode *p2 = head;

        while (true) {
            if (p2->next == nullptr) {
                // 奇数
                return p1;
            }
            p2 = p2->next;
            if (p2->next == nullptr) {
                // 偶数
                return p1;
            }
            p2 = p2->next;
            p1 = p1->next;
        }
    }

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
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
