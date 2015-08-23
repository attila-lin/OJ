#include <iostream>
#include <stdio.h>

typedef long long int lli;

#define max(a, b)  (a) > (b) ? (a) : (b)
#define min(a, b)  (a) > (b) ? (b) : (a)


int main(int argc, char const *argv[])
{
  /* code */
  int n = 0;
  int a[1000];
  int x;
  int res = 0;
  std::cin >> n;
  for(int i = 0; i < n; i++)
  {
	  std::cin >> a[i];
  }
  for(int i = 0; i < n; i++)
  {
	  std::cin >> x;
	  if(x > a[i])
	  	res += x - a[i];
	  else
	  	res += a[i] - x;
  }
  return 0;
}