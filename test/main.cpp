#include <iostream>

#include "ArrayList.h"

using namespace std;



void printIt(ArrayList& list);


int main() {
    ArrayList list;

    list.add(9);
    list.add(3);
    list.add(4);
    list.add(0, -5);
    cout << "removed: " << list.remove(2) << endl;
    printIt(list);

    cout << "working";


}


void printIt(ArrayList& list) {
    cout << "size: " << list.getSize() << ", cap: " << list.getCapacity() << endl;
    cout << "list: ";
    for (int i = 0; i < list.getSize(); i++) {
        cout << list.get(i) << ", ";
    }
    cout << endl;
}