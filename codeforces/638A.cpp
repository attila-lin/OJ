#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>
#include <stack>
#include <map>

using namespace std;

# define PI           3.14159265358979323846

typedef long long int lli;

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n;
  cin >> n >> m;

  if(m % 2 == 1)
  {
    l = (m + 1) / 2;
  }
  else{
    l = (n - m) / 2 + 1;
  }

  cout << l << endl;


  return 0;
}
