

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
    lli i,j,k,l,m,n, max=0,sum=0;
    std::cin >> n;

    int *num=new int[n];
  	for(int i=0; i<n; i++)
  	{
  		cin>>num[i];
  		sum+=num[i];
  		if(num[i]>max)
  			max=num[i];
  	}
  	cout<<(2*max)-sum+1 << endl;

    return 0;
  }
