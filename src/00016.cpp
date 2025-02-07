#include <iostream>
#include <list>
#include <numeric>

using std::accumulate;
using std::list;
using std::cout;
using std::endl;

void multiply(list<int>& lst, int fact){
    int temp,carry=0;
    for(auto ite=lst.begin();ite!=lst.end();++ite){
        temp = *ite * fact + carry;
        *ite = temp % 10;
        carry = temp/10;
    }
    if(carry) lst.push_back(carry);
}
int main(){
    list<int> lst{1};
    for(auto i=0;i<1000;++i){
        multiply(lst,2);        
    }
    int ret = accumulate(lst.begin(),lst.end(),0);
    cout << ret << endl;
    return 0;
}
