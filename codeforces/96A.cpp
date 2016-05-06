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

#define max(a, b)  (a) > (b) ? (a) : (b)
#define min(a, b)  (a) > (b) ? (b) : (a)

# define PI           3.14159265358979323846

typedef long long int lli;

int main(int argc, char const *argv[])
{
  char c;
  int count = 1;
  scanf("%c",&c);
  char last = c;
  while(scanf("%c",&c) && c == '1' || c == '0')
  {
  	if(last == c){
		count ++;  	
  		if(count >= 7)
		{
			cout << "YES" << endl;
			return 0;
		}
  	}
  	else{
	  	
	  	count = 1;
	  	last = c;
	  }
  	
  }
  
  cout << "NO" << endl;

  return 0;
}
