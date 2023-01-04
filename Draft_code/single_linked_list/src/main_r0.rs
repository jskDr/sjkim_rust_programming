struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct SinglyLinkedList<T> {
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
        match self.root {
            Some(ref mut n) => {
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
                        print!("{} => ", node.data);
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

fn _run1() {
    let node = Node {
        data: 1,
        next: None,
    };
    println!("{}", node.data);

    let slist = SinglyLinkedList::<i32> {
        root: Some(Box::new(node))
    };
    if let Some(node) = slist.root {
        println!("{}", node.data);
    }
}

fn main() {
    println!("Singely Linked List");
    let mut slist = SinglyLinkedList::<i32>::new();
    for i in 0..10 {
        slist.append(i);
    }
    slist.print();
}