#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>

# define PI           3.14159265358979323846


typedef long long int lli;

int main(int argc, char const *argv[])
{
  double d, h, v, e;
  std::cin >> d >> h >> v >> e;
  double area = pow(d/2, 2)*PI;
  double speed = v / area;
  if(speed < e)
  {
    std::cout << "NO" << std::endl;
    return 0;
  }

  std::cout << "YES" << std::endl;
  std::cout << std::fixed << std::setprecision(12) << h / (speed - e) << std::endl;

  return 0;
}
