#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>
#include <vector>
#include <stack>
#include <map>

using namespace std;

# define PI           3.14159265358979323846

typedef long long int lli;

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n,a,b;
  cin >> n >> a >> b;

  if((a+b) % n <= 0)
    cout << (a+b) % n + n << endl;
  else
    cout << (a+b) % n << endl;

  return 0;
}
