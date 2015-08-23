#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  /* code */
  int  k, n, w;
  std::cin >> k >> n >> w;
  int res = (1+w)*w/2 * k - n;
  if(res < 0) res = 0;
  std::cout << res  << std::endl;
  return 0;
}