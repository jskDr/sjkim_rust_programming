struct Node<T> {
    data: T, // i32, f32, ...
    next: Option<Box<Node<T>>>, // None, Some(x)
}

fn run_node() {
    let node = Node {
        data: 10,
        next: None
    };
    println!("{}", node.data);
}




struct SinglyLinkedList<T> { // T* root
    root: Option<Box<Node<T>>>
}

impl<T: std::fmt::Display> SinglyLinkedList<T> {
    fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList::<T> {
            root: None
        }
    }

    fn append(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: None
        };
        match self.root { // Option<&mut Some<T>> X
            Some(ref mut n) => { // Some(x) => ref mut
                let mut node = n;
                loop {
                    // let mut node = n; // &mut type
                    match node.next {
                        Some(ref mut n) => node = n,
                        None => {
                            node.next = Some(Box::new(new_node));
                            break
                        }
                    }
                }
            },
            None => self.root = Some(Box::new(new_node))
        }
    }

    fn print(&self) {
        if let Some(ref n) = self.root {
            let mut node = n;
            loop {
                match node.next {
                    Some(ref n) => {
                        print!("{} -> ", node.data);
                        node = n
                    },
                    None => {
                        println!("{}", node.data);
                        break
                    }
                }
            }
        }
    }
}

fn run_single_linked_list() {
    println!("Singely Linked List");
    let mut slist = SinglyLinkedList::<i32>::new();
    for i in 0..10 {
        slist.append(i);
    }
    slist.print();
}

fn main() {
    run_node();
    run_single_linked_list();
}