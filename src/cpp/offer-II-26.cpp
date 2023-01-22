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
    void reorderList(ListNode *head) {
        vector<ListNode *> arr;
        while (head != nullptr) {
            arr.push_back(head);
            head = head->next;
        }

        int l = 0;
        int r = arr.size() - 1;
        while (l < r) {
            arr[l]->next = arr[r];
            arr[r]->next = arr[l + 1];
            l++;
            r--;
        }
        arr[arr.size() / 2]->next = nullptr;
    }
};

int main() {
    // Solution solu;
    // solu
    return 0;
}
