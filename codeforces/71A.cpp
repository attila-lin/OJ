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
  std::string x;
  for (int i = 0; i < n; ++i)
  {
    std::cin >> x;
    if(x.length() > 10)
    {
      std::cout << x[0] << x.length() - 2 << x[x.length() - 1] << std::endl;
    }
    else
    {
      std::cout << x << std::endl;
    }
  }


  return 0;
}