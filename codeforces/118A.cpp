#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>

typedef long long int lli;

int main(int argc, char const *argv[])
{
  std::string n;
  std::cin >> n;

  for (int i = 0; i < n.length(); ++i)
  {
    // "A", "O", "Y", "E", "U", "I"
    if(n[i] == 'A' || n[i] == 'a'
      ||n[i] == 'O' || n[i] == 'o'
      ||n[i] == 'Y' || n[i] == 'y'
      ||n[i] == 'E' || n[i] == 'e'
      ||n[i] == 'U' || n[i] == 'u'
      ||n[i] == 'I' || n[i] == 'i')
    {
      continue;
    }
    else{
      std::cout << '.' <<char(tolower(n[i])) ;
    }
  }
  std::cout << std::endl;

  return 0;
}