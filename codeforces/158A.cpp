#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>

typedef long long int lli;
int a[100];
int main(int argc, char const *argv[])
{
  int n, k;
  std::cin >> n >> k;
  int tmp, t;
  for (int i = 0; i < n; ++i)
  {
    std::cin >> tmp;
    a[i] = tmp;
    if(i == k-1)
      t = tmp;
  }
  int res = 0;

  for (int i = 0; i < n; ++i)
  {
    if(a[i] > 0 && a[i] >= t)
      res ++;
  }
  std::cout << res << std::endl;
  return 0;
}