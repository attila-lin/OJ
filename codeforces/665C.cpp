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

char s[200004];

typedef long long int lli;

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n;
  // cin >> n;
  scanf("%s", s);
  for (int i = 1; s[i]!='\0'; i++) {
    if( s[i]==s[i-1] )
    {
      for(s[i] = 'a'; s[i] == s[i-1] || s[i] == s[i+1]; s[i]++);
    }
  }
  printf("%s\n", s);

  return 0;
}
