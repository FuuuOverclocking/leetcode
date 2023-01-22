#include "default.h"

struct Time {
    int hour;
    int min;
    bool operator<(const Time &t) {
        if (hour < t.hour) {
            return true;
        } else if (hour == t.hour) {
            return min < t.min;
        }
        return false;
    }
    int operator-(const Time &t) {
        return (hour - t.hour) * 60 + min - t.min;
    }
};

class Solution {
public:
    int findMinDifference(vector<string> &timePoints) {
        if (timePoints.size() > 1440) {
            return 0;
        }
        vector<Time> time_arr;
        time_arr.reserve(timePoints.size());
        transform(timePoints, time_arr);

        sort(time_arr.begin(), time_arr.end());

        int ret = INT_MAX;
        for (int i = 0; i < time_arr.size() - 1; i++) {
            ret = min(ret, time_arr[i + 1] - time_arr[i]);
        }
        time_arr.front().hour += 24;
        ret = min(ret, time_arr.front() - time_arr.back());

        return ret;
    }

    void transform(vector<string> &timePoints, vector<Time> &time_arr) {
        for (auto &str : timePoints) {
            Time t{0, 0};
            t.hour = (str[0] - '0') * 10 + (str[1] - '0');
            t.min = (str[3] - '0') * 10 + (str[4] - '0');
            time_arr.push_back(t);
        }
    }
};

int main() {
    Solution solu;
    vector<string> timePoints {"23:59", "00:00"};
    solu.findMinDifference(timePoints);
    return 0;
}
