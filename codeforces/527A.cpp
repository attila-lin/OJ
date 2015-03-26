#include <iostream>
#include <stdio.h>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  lli a, b, res = 0;
  std::cin >> a >> b;
  lli max = std::max(a,b);
  lli min = std::min(a,b);
  lli tmp;
  while(max % min != 0){
    res += max / min;
    tmp = max % min;
    max = std::max(tmp, min);
    min = std::min(tmp, min);
    // std::cout << res << std::endl;
  }
  res += max / min;
  std::cout << res << std::endl;
  return 0;
}