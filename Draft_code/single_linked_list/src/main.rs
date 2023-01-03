struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

fn get_list_len<T>(root: &Node<T>) -> usize {
    // As root is not None and no Option type, root must have a value
    // This is an important point to distinguish between Rust and C/C++
    let mut len = 0;
    let mut node = root;
    loop {
        len += 1;
        match node.next {
            // this is not a reference, but a value
            // Some(n) => node = n, so use ref to get a reference as below
            Some(ref n) => node = n, 
            None => break,
        }
    }
    len
}

#[test]
fn test_node() {
    // Generate list and print directly
    let a = Node {
        data: 3.4,
        next: None,
    }; 
    let b = Node {
        data: 5.6,
        next: Some(Box::new(a)),
    };
    let c = Node {
        data: 7.8,
        next: Some(Box::new(b)),
    };
    println!("c.data = {}", c.data);
    println!("c.next.data = {}", c.next.as_ref().unwrap().data);
    println!("c.next.next.data = {}", c.next.as_ref().unwrap().next.as_ref().unwrap().data);

    let ln = get_list_len(&c);
    println!("list length = {}", ln);
    assert_eq!(ln, 3, "list length is not 3");
}

struct ListNode<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> ListNode<T> {
    fn new() -> ListNode<T> {
        ListNode::<T> {
            root: None,
        }
    }

    fn get_list_len(&self) -> usize {
        let mut len = 0;
        match self.root {
            Some(ref n) => {
                let mut node = n;
                loop {
                    len += 1;
                    match node.next {
                        Some(ref n) => node = n,
                        None => break,
                    }
                }
            },
            None => {},
        }
        len
    }

    fn print_list(&self) {
        match self.root {
            Some(ref n) => {
                let mut node = n;
                loop {
                    print!("{}", node.data);
                    match node.next {
                        Some(ref n) => {
                            node = n;
                            print!(" -> ")
                        },
                        None => {
                            println!();
                            break
                        }
                            
                    }                    
                }
            },
            None => {}
        }
    }

    fn append_node(&mut self, data: T) {
        let new_node = Node {
            data: data,
            next: None,
        };
        match self.root {
            Some(ref mut n) => {
                let mut node = n;
                loop {
                    match node.next {
                        Some(ref mut n) => node = n,
                        None => {
                            node.next = Some(Box::new(new_node));
                            break;
                        }
                    }
                }
            },
            None => {
                self.root = Some(Box::new(new_node));
            }
        }
    }
}

#[test]
fn test_list() {
    let list_node_i32 = ListNode::<i32> {
        root: None,
    };
    match list_node_i32.root {
        Some(ref n) => println!("list_node_i3.root.data = {}", n.data),
        None => println!("list_node_i3.root is None"),
    }
    let mut list_node_f64 = ListNode::<f64>::new();
    match list_node_f64.root {
        Some(ref n) => println!("list_node_f64.root.data = {}", n.data),
        None => println!("list_node_f64.root is None"),
    }
    let ln = list_node_f64.get_list_len();
    println!("list_node_i32 length = {}", ln);
    assert_eq!(ln, 0, "list_node_i32 length is not 0");

    list_node_f64.append_node(3.4);
    list_node_f64.append_node(5.6);
    list_node_f64.append_node(7.8);
    let ln = list_node_f64.get_list_len();
    println!("list_node_f64 length = {}", ln);
    assert_eq!(ln, 3);
    println!("Print list_node_f64 length");
    list_node_f64.print_list();

}

fn main() {
    println!("For testing, use cargo test");
}
