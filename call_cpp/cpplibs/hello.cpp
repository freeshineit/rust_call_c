#include <iostream>
using namespace std;


extern "C" {
    void hello();
}

void hello() {
    cout << "Hello, world cpp!!!!!" << endl;
}