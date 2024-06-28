#ifndef ArrayList_h
#define ArrayList_h

class ArrayList {
private:
    int* array;
    int size;
    int capacity;

    void expand();
    void shrink();
    void checkSize();
    bool needGrow();
    bool needShrink();
public:
    ArrayList();
    ArrayList(int size);
    ~ArrayList();

    void add(int val);
    void add(int spot, int val);
    int remove();
    int remove(int spot);
    int get(int spot);
    int set(int spot, int val);

    int getSize();
    int getCapacity();
};

#endif