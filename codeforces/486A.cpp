#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  lli n;
  std::cin >> n;
  if(n % 2 == 0)
    std::cout << n / 2 << std::endl;
  else
    std::cout << -1 * (n+1)/2 << std::endl;

  return 0;
}