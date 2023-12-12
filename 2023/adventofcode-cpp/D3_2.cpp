#include <iostream>
#include <fstream>
#include <vector>
#include <string.h>

using namespace std;
ifstream fin(".in");

void showMatrix (char a[][256], int n) {
    for (int i = 0; i < n; i++)
        cout << a[i] << '\n';
}

int main()
{
    char buffer[256];
    char a[256][256];
    const int dx[] = {-1, -1, -1, 0, 0, 1, 1, 1},
        dy[] = {-1, 0, 1, -1, 1, -1, 0, 1};
    int n = 0;
    while (fin.getline(buffer, 256)) {
        //cout << buffer << endl;
        strcpy(a[n++], buffer);
    }
    
    unsigned long long int sum = 0;
    
    //const char symbols[] = {"!@#$%^&*()_-=[]+"};
    
    for (int i = 0; i < n; i++) 
        for (int j = 0; a[i][j]; j++) 
            if (a[i][j] == '*') {
            vector<int> vec;
                for (int k = 0; k < 8; k++) {
                    
                    int ii = i + dx[k], jj = j + dy[k];
                    if (isdigit(a[ii][jj])) {
                        // find full number
                        int right = jj, left = jj;
                        while (isdigit(a[ii][right])) right++;
                        while (isdigit(a[ii][left])) left--;
                        unsigned long long int num = 0, power = 1;
                        for (int p = right - 1; p > left; p--) {
                            num += (int) (a[ii][p] - '0') * power;
                            power *= 10;
                        }
                        printf("Found number: %d at index i=%d, j=%d\n", num, ii, left + 1);
                        vec.push_back(num);
                        for (int p = left + 1; p < right; p++) a[ii][p] = '.';
                    }
                }
                unsigned long long int prod = 1;
                for (int i : vec) prod *= i;
                sum += prod;
            }
        
        showMatrix(a, n);
        printf("Sum is %d \n", sum);
    
    fin.close();

    return 0;
}