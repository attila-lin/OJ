#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>
#include <stack>
#include <map>

using namespace std;

#define max(a, b)  (a) > (b) ? (a) : (b)
#define min(a, b)  (a) > (b) ? (b) : (a)

# define PI           3.14159265358979323846

typedef long long int lli;

char a[1005][6];

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n;
  cin >> n;
  int ret = 6;
  for (lli i = 0; i < n; i++) {
    scanf("%s", a[i]);
    for (lli j = 0; j < i; j++) {
      int r = 0;
      for(int k = 0; k < 6; k++)
        r += a[j][k] != a[i][k];

      // printf("rrr %d\n", r);
      ret = min(ret, (r+1) / 2 - 1 );
    }
  }
  cout << ret << endl;

  return 0;
}
