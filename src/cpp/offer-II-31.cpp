#include "default.h"

struct Node {
    Node *prev;
    Node *next;
    int key;
    int val;
};

void link(Node *n1, Node *n2) {
    n1->next = n2;
    n2->prev = n1;
}

class LRUCache {
    uint len;
    uint cap;
    Node *arr;
    Node *head;
    Node *tail;
    unordered_map<int /*key*/, Node *> map;

public:
    LRUCache(int capacity) {
        len = 0;
        cap = capacity;
        arr = new Node[cap];
        head = nullptr;
        tail = nullptr;
    }

    int get(int key) {
        auto iter = map.find(key);
        if (iter == map.end()) {
            return -1;
        }
        auto node = iter->second;
        if (node->next != nullptr && node->prev != nullptr) {
            link(node->prev, node->next);
            link(node, head);

            node->prev = nullptr;
            head = node;
        } else if (node->prev != nullptr) {
            tail = node->prev;
            tail->next = nullptr;
            node->prev = nullptr;
            link(node, head);
            head = node;
        }
        return node->val;
    }

    void put(int key, int value) {
        if (len == 0) {
            arr[len++] = Node{nullptr, nullptr, key, value};
            head = tail = &arr[0];
            map.insert({key, head});
            return;
        }
        if (get(key) != -1) {
            head->val = value;
            return;
        }
        if (len < cap) {
            arr[len++] = Node{nullptr, nullptr, key, value};
            link(&arr[len - 1], head);
            head = &arr[len - 1];
            map.insert({key, head});
            return;
        }
        if (len == 1) {
            map.erase(tail->key);
            map.insert({key, tail});
            tail->key = key;
            tail->val = value;
            return;
        }

        map.erase(tail->key);

        auto node = tail;
        tail = tail->prev;
        tail->next = nullptr;

        node->key = key;
        node->val = value;
        node->prev = nullptr;
        link(node, head);
        head = node;

        map.insert({key, node});
    }
};

int main() {
    LRUCache c(2);
    c.put(2, 1);
    c.put(1, 1);
    c.put(2, 3);
    c.put(4, 1);
    c.get(1);
    c.get(2);

    return 0;
}
