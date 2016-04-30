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

# define PI           3.14159265358979323846

typedef long long int lli;

using namespace std;


// 超时
// int main(int argc, char const *argv[])
// {
//   int n;
//   cin >> n;
//
//   vector<string> v;
//   vector<string>::iterator it;
//
//   string name;
//   for (int i = 0; i < n; i++) {
//     cin >> name;
//
//     for(it=v.begin();it!=v.end();++it)
//     {
//       if(*it == name)
//       {
//         v.erase(it);
//         break;
//       }
//     }
//     v.insert(v.begin(), name);
//   }
//
//   for (it=v.begin();it!=v.end();++it) {
//     cout << *it << endl;
//   }
//
//   return 0;
// }


map <string,bool> arr;

string ch[200005];

int main()
{
    lli i,j,k,l,m,n;
    cin >> n;
    for(i=0;i<n;i++)
    {
        cin >> ch[i];
    }
    for(i=n-1;i>=0;i--)
    {
        if(!arr[ch[i]])
        {
            cout << ch[i] << endl;
            arr[ch[i]]=1;
        }
    }

}
