// Typescript recommend to write member functions inside the class
class NodeT<T> {
  data: T;
  next: NodeT<T> | null;
  
  constructor(data: T) {
    this.data = data;
    this.next = null;
  }

  get_data(): T {
    return this.data;
  }

  set_data(data: T): void {
    this.data = data;
  }
}

function run_node() {
    var node = new NodeT(10);
    console.log(node.get_data());
    node.set_data(20);
    console.log(node.get_data());
}

// main part
run_node();

