#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  int n1,n2;
  std::cin >> n1 >> n2;
  int k,m;
  std::cin >> k >> m;
  long tmp,tmp1, tmp2;
  for(int i = 0; i < n1; i++)
  {
    std::cin >> tmp;
    if(i == k-1)
      tmp1 = tmp;
  }
  for(int i = 0; i < n2; i++)
  {
    std::cin >> tmp;
    if(i == n2 - m)
      tmp2 = tmp;
  }
  //std::cout << tmp1 << tmp2 << std::endl;
  if(tmp1 < tmp2)
    std::cout << "YES" <<std::endl;
  else
    std::cout << "NO" << std::endl;

  return 0;
}