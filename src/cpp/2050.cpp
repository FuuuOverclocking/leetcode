#include <cstdint>
#include <vector>

using namespace std;

class Solution {
    vector<int> course_min_time;
    vector<vector<int>> adj;

    int get_course_min_time(int idx, vector<int> &time) {
        if (course_min_time[idx] != 0) {
            return course_min_time[idx];
        }

        course_min_time[idx] = time[idx];
        int additional_time = 0;
        for (auto i : adj[idx]) {
            additional_time = max(additional_time, get_course_min_time(i, time));
        }
        course_min_time[idx] += additional_time;
        return course_min_time[idx];
    }

public:
    int minimumTime(int n, vector<vector<int>> &relations, vector<int> &time) {
        course_min_time.resize(n, 0);
        adj.resize(n);

        for (auto &v : relations) {
            adj[v[1] - 1].push_back(v[0] - 1);
        }

        int ret = 0;
        for (int i = 0; i < n; i++) {
            ret = max(ret, get_course_min_time(i, time));
        }

        return ret;
    }
};

int main() {
    return 0;
}