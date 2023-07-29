#include "default.h"
#include <cstddef>

class MovingAverage {
    int *ring;
    int tail = 0;
    int size = 0;
    const int cap;
    int sum = 0;

public:
    /** Initialize your data structure here. */
    MovingAverage(int size) : cap(size) { ring = new int[cap](); }

    double next(int val) {
        sum += val;

        if (size == cap) {
            sum -= ring[tail];
        } else {
            size += 1;
        }
        ring[tail] = val;
        tail = (tail + 1) % cap;

        return static_cast<double>(sum) / static_cast<double>(size);
    }
};

/**
 * Your MovingAverage object will be instantiated and called as such:
 * MovingAverage* obj = new MovingAverage(size);
 * double param_1 = obj->next(val);
 */

int main() { return 0; }
