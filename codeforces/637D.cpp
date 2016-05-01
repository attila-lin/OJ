#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>
#include <vector>
#include <stack>
#include <map>

using namespace std;

#define max(a, b)  (a) > (b) ? (a) : (b)
#define min(a, b)  (a) > (b) ? (b) : (a)

# define PI           3.14159265358979323846

typedef long long int lli;

lli a[200005];
//
// int main(int argc, char const *argv[])
// {
//   lli i,j,k,l,m,n,s,d;
//   cin >> n >> m >> s >> d;
//   a[n] = m;
//   for (lli i = 0; i < n; i++) {
//     scanf("%lld", &a[i]);
//   }
//
//   sort(a, a+n+1);
//
//   bool possible = true;
//   k = 0;
//   for (lli i = 0; i < n+1; i++) {
//     // can run
//     if(a[i] - k - 1 >= s && i % 2 == 0)
//     {
//       printf("RUN %lld\n", a[i] - k - 1);
//       k = a[i] - 1;
//       continue;
//     }
//     // can jump
//     if(a[i] - k + 1 <= d && i % 2 == 1)
//     {
//       printf("JUMP %lld\n", a[i] - k + 1);
//       k = a[i] + 1;
//       continue;
//     }
//     possible = false;
//     if(!possible)
//     {
//         cout << "IMPOSSIBLE" << endl;
//         break;
//     }
//   }
//
//   if(possible)
//   {
//     k = 0;
//     for (lli i = 0; i < n+1; i++) {
//       // can run
//       if(a[i] - k - 1 >= s && i % 2 == 0)
//       {
//         printf("RUN %lld\n", a[i] - k - 1);
//         k = a[i] - 1;
//         continue;
//       }
//       // can jump
//       if(a[i] - k + 1 <= d && i % 2 == 1)
//       {
//         printf("JUMP %lld\n", a[i] - k + 1);
//         k = a[i] + 1;
//         continue;
//       }
//
//     }
//   }
//
//   return 0;
// }
