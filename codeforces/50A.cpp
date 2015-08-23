#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  int n,m;
  std::cin >> n >> m;
  int res = 0;
  if(n % 2 == 0 || m % 2 == 0)
  {
    res = n * m / 2;
  }
  else
  {
    res = (n * m - 1) / 2;
  }
  std::cout << res << std::endl;

  return 0;
}