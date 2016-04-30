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

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n;
  cin >> n;
  vector<int> v;

  for (i = 0; i < n; i++) {
    cin >> j;
    v.push_back(j);
  }

  sort(v.begin(), v.end());
  // 去重
  vector<int>::iterator iter = unique(v.begin(),v.end());
  v.erase(iter,v.end());

  bool ret = false;
  // 各種情況的判斷
  if(v.size() >= 3)
    for(i = 0; i < v.size() - 2; i++)
    {
      if(v[i+2] - v[i+1] == 1 && v[i+1] - v[i] == 1)
        {
          ret = true;
          break;
        }
    }
  if(ret)
    cout << "YES" << endl;
  else
    cout << "NO" << endl;

  return 0;
}
