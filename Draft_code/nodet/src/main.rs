struct NodeT<T> {
    data: T, // i32, f32, ...
    next: Option<Box<NodeT<T>>>, // None, Some(x)
}

impl<T> NodeT<T> {
    fn new(data: T) -> NodeT<T> {
        NodeT {
            data: data,
            next: None
        }
    }
    fn get_data(&self) -> &T {
        &self.data
    }
    fn set_data(&mut self, data: T) { // move data
        self.data = data;
    }
}

fn run_node() {
    let mut node = NodeT::new(10);
    println!("{:?}", node.get_data());
    node.set_data(20);
    println!("{:?}", node.get_data());
}

fn main() {
    run_node();
}
