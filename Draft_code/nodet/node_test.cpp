#incldue <iostream>

template<class T>
class PointT {
public:
  T x;
  T y;

  PointT(T x, T y): x(x), y(y) {};
  PointT(const PointT<T>& p): x(p.x), y(p.y) {};
  PointT(PointT<T>&& p): x(std::move(p.x)), y(std::move(p.y)) {};
};

template<class T>
class NodeT {
private:
  T data;
  NodeT* next;

public:
  NodeT(T data) {
    this->data = data;
    this->next = nullptr;
  }

  T get_data() {
    return this->data;
  }

  void set_data(T data) {
    this->data = data;
  }
};

void run_node() {
  PointT<int> p(1, 2);
  NodeT<PointT<int>> node = NodeT<PointT<int>>(p);
  cout << node.get_data().x << ", " << node.get_data().y << endl;
}

int main() {
    run_code();
}