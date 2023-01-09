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

template<class T>
struct PointT {
    T x, y;
};

void run_pointt() {
    // PointT<int> p;
    // PointT<int> p = PointT<int>();
    // p.x = 3;
    // p.y = 4;
    PointT<int> p = {1, 2};
    NodeT<PointT<int>> node_point = NodeT<PointT<int>>(p);
    cout << node_point.get_data().x << ", " << 
        node_point.get_data().y << endl;

    cout << p.x << ", " << p.y << endl;
}

int main() {
    run_node();
    run_pointt();
    return 0;
}