#include <vector>

using namespace std;

class Solution {
    vector<vector<long long>> mem;
    long long work(long long n, long long startsWith) {
        if (n == 1) return 1;
        if (mem[n][startsWith] != 0) return mem[n][startsWith];

        long long ret = 0;
        switch (startsWith) {
        case 0:
            ret += work(n - 1, 1);
            ret = ret % (1000000007);
            break;
        case 1:
            ret += work(n - 1, 0);
            ret = ret % (1000000007);
            ret += work(n - 1, 2);
            ret = ret % (1000000007);
            break;
        case 2:
            ret += work(n - 1, 0);
            ret = ret % (1000000007);
            ret += work(n - 1, 1);
            ret = ret % (1000000007);
            ret += work(n - 1, 3);
            ret = ret % (1000000007);
            ret += work(n - 1, 4);
            ret = ret % (1000000007);
            break;
        case 3:
            ret += work(n - 1, 2);
            ret = ret % (1000000007);
            ret += work(n - 1, 4);
            ret = ret % (1000000007);
            break;
        case 4:
            ret += work(n - 1, 0);
            ret = ret % (1000000007);
            break;
        }

        mem[n][startsWith] = ret;
        return ret;
    }

public:
    int countVowelPermutation(int n) {
        for (long long i = 0; i <= n; i++) {
            mem.emplace_back(5, 0);
        }
        long long ret = 0;
        ret += work(n, 0);
        ret = ret % (1000000007);
        ret += work(n, 1);
        ret = ret % (1000000007);
        ret += work(n, 2);
        ret = ret % (1000000007);
        ret += work(n, 3);
        ret = ret % (1000000007);
        ret += work(n, 4);
        ret = ret % (1000000007);
        return ret;
    }
};

int main() {
    return 0;
}