#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>        //務必引用algorithm

typedef long long int lli;
int a[3000];

int main(int argc, char const *argv[])
{
  /* code */
  int n;
  std::cin >> n;
  int sum = 0;
  int tmp = 0;
  for (int i = 0; i < n; ++i)
  {
    std::cin >> a[i];
  }
  std::sort(a,a+n);

  int res = 0;
  int max = -1;
  for (int i = 0; i < n; ++i)
  {
    if(a[i] > max)
    {
      max = a[i];
    }
    else
    {
      res += (max - a[i]) + 1;
      max += 1;
    }
  }
  std::cout << res << std::endl;
  return 0;
}