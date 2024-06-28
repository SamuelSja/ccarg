#include "ArrayList.h"

//for testing
#include <iostream>
using namespace std;

ArrayList::ArrayList() {
    size = 0;
    capacity = 2;
    array = new int[capacity];
}

ArrayList::ArrayList(int size) {
    this->size = 0;
    capacity = size;
    array = new int[capacity];
}

ArrayList::~ArrayList() {
    delete [] array;
}

void ArrayList::expand() {
    capacity *= 2;
    int* newArray = new int[capacity];
    for (int i = 0; i < size; i++) {
        newArray[i] = array[i];
    }

    delete [] array;
    array = newArray;
}

void ArrayList::shrink() {
    capacity /= 2;
    int* newArray = new int[capacity];
    for (int i = 0; i < size; i++) {
        newArray[i] = array[i];
    }

    delete [] array;
    array = newArray;
}

void ArrayList::add(int val) {
    if (size == capacity) {
        expand();
    }

    array[size] = val;
    size++;
}

void ArrayList::add(int spot, int val) {
    size++;
    checkSize();

    int curSpot = spot;
    int cur = val;
    while (curSpot < size) {
        int temp = array[curSpot];
        array[curSpot] = cur;
        cur = temp;
        curSpot++; 
    }
}

int ArrayList::remove() {
    size--;
    return array[size];

    checkSize();
}

int ArrayList::remove(int spot) {
    int curSpot = spot;
    int ans = array[spot];

    while (curSpot < size - 1) {
        array[curSpot] = array[curSpot + 1];
        curSpot++;
    }

    size--;
    checkSize();

    return ans;    
}

int ArrayList::get(int spot) {
    return array[spot];
}

int ArrayList::set(int spot, int val) {
    int ans = get(spot);
    array[spot] = val;
    return ans;
}

int ArrayList::getSize() {
    return size;
}

int ArrayList::getCapacity() {
    return capacity;
}

void ArrayList::checkSize() {
    if (needGrow()) {
        expand();
    } else if (needShrink()) {
        shrink();
    }
}

bool ArrayList::needGrow() {
    return size > capacity;
}

bool ArrayList::needShrink() {
    return size == capacity / 4;
}

