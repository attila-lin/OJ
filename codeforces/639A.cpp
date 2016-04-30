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

# define PI           3.14159265358979323846

typedef long long int lli;

const int MX = 150005;
// 空间换时间
int a[MX], b[MX], c[MX];

int main(int argc, char const *argv[]) {
  lli i,j,k,l,m,n,q;
  scanf("%lld%lld%lld", &n, &k, &q);
  for(i = 0; i < n; i ++)
    scanf("%d", a+i);
  for(i = 0; i < q; i ++)
  {
    int g, id;
    scanf("%d %d", &g, &id);
		id--;

    if(g == 1)
    {
      b[id] = 1;
      c[0] = a[id];
      sort(c, c+k+1);
    }
    else if(b[id] == 1 && a[id] >= c[1]) cout << "YES" << endl;
    else cout << "NO" << endl;
  }

  return 0;
}

// 超时
//
// vector<pair<lli, lli> > v;
// vector<pair<lli, lli> > o;
//
//
// bool maxtomin(const pair<lli, lli> & m1, const pair<lli, lli> & m2) {
//         return m1.first > m2.first;
// }
//
// int main(int argc, char const *argv[])
// {
//   lli i,j,k,l,m,n,q;
//   cin >> n >> k >> q;
//
//
//   for (i = 0; i < n; i++) {
//     cin >> j;
//     v.push_back(make_pair(j, i+1));
//   }
//
//   bool needSort = false;
//   for (i = 0; i < q; i++) {
//     cin >> j >> m;
//     if(j == 1)
//     {
//       o.push_back(v[m-1]);
//       needSort = true;
//     }
//     else{
//       if(needSort)
//         sort(o.begin(), o.end(), maxtomin);
//       needSort = false;
//
//       bool ret = false;
//       for (l=0; l < o.size();l++)
//       {
//         // cout << o[l].second << l << k << endl;
//         if(l < k && o[l].second == m)
//         {
//           ret = true;
//           break;
//         }
//       }
//       if(ret)
//         cout << "YES" << endl;
//       else
//         cout << "NO" << endl;
//
//     }
//   }
//
//   // sort(v.begin(), v.end());
//
//   return 0;
// }
