#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <string.h>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  int n;
  std::cin >> n;

  int a[4];
  memset (a,0,4);

  int tmp = 0;
  for (int i = 0; i < n; ++i)
  {
    std::cin >> tmp;
    a[tmp - 1] ++;
  }
  int res = 0;
  res += std::max(a[0],a[2]) + a[3];
  if(a[1] % 2 == 0)
    res += a[1]/2;
  else
    res += (a[1]+1)/2;
  std::cout << res << std::endl;

  return 0;
}