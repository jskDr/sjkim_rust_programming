#include <iostream>
using namespace std;

template<class T>
class NodeT {
    public:
    T data;
    NodeT *next;
    NodeT(T data) {
        this->data = data;
        this->next = nullptr;
    }
    T get_data()
    {
        return this->data;
    }
    void set_data(T data)
    {
        this->data = data;
    }
};

void run_node() {
    NodeT<int> node = NodeT<int>(10);
    cout << node.get_data() << endl;
    node.set_data(20);
    cout << node.get_data() << endl;
}

struct Point {
    int x, y;
};

void run_point() {
    Point p = {1, 2};
    NodeT<Point> node_point = NodeT<Point>(p);
    cout << node_point.get_data().x << ", " << 
        node_point.get_data().y << endl;
    cout << p.x << ", " << p.y << endl;
}

int main() {
    run_node();
    run_point();
    return 0;
}