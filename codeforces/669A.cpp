#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  int n;
  std::cin >> n;
  int res = (n / 3) * 2;
  int mod = n % 3;
  if(n % 3 != 0)
    res ++;

  std::cout << res << std::endl;

  return 0;
}
