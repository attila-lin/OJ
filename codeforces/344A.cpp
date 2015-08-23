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
  int last = 0, tmp;
  int res = 0;
  for(int i = 0; i < n; i++)
  {
    std::cin >> tmp;
    if(tmp != last)
      res ++;

    last = tmp;
  }
  std::cout << res << std::endl;
  return 0;
}