#include <iostream>
#include <stdio.h>
#include <cstdlib>
#include <iostream>
#include <algorithm>
#include <string>
#include <math.h>
#include <iomanip>
#include <stack>
#include <vector>
#include <map>

using namespace std;

# define PI           3.14159265358979323846

typedef long long int lli;



typedef struct team{
  string name;
  int score;
}Team;

vector<Team> a[10003];

bool maxtomin(const Team & m1, const Team & m2) {
        return m1.score > m2.score;
}

int main(int argc, char const *argv[])
{
  lli i,j,k,l,m,n;
  cin >> n >> m;
  vector<Team> v;
  vector<Team>::iterator it;

  string name;
  int score;

  for (i = 0; i < n; i++) {
    cin >> name >> j >> score;
    Team t;
    t.name = name;
    t.score = score;
    a[j].push_back(t);
    // cout <<"en" << a[j].begin()->name << endl;
  }

  for (i = 1; i <= m; i++) {
    sort(a[i].begin(), a[i].end(), maxtomin);
    if(a[i].size() > 2 && a[i][1].score == a[i][2].score)
      cout << "?" << endl;
    else
      cout << a[i][0].name << " " << a[i][1].name << endl;
  }

  return 0;
}
