// declar to use queue vector
use std::collections::VecDeque;

// Binary Tree Node
struct BTN {
    data: i32, // i8, i16, i32, i64, i128, isize
    left: Option<Box<BTN>>, // usize 
    right: Option<Box<BTN>>
}

// implementation for BTN
impl BTN {
    fn new(data: i32) -> BTN {
        BTN {
            data,
            left: None,
            right: None
        }
    }
}

fn print_preorder(root: &BTN) {
    // define a queue vector
    let mut queue: VecDeque<&BTN> = VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        print!("{} ", node.data);
        if let Some(left) = &node.left {
            queue.push_back(left);
        }
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
    println!();
}

fn main() {
    today_tip();

    println!("Preorder of BTN");
    let mut root = BTN::new(1);
    let left = BTN::new(2);
    let right = BTN::new(3);
    let left_left = BTN::new(4);
    let left_right = BTN::new(5);
    let right_left = BTN::new(6);
    let right_right = BTN::new(7);
    /*
          1
        /   \
       2     3
      / \   /  \ 
     4   5 6    7
    */
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    if let Some(r_left) = &mut root.left // Some(Box::new(left));
    {
        r_left.left = Some(Box::new(left_left));
        r_left.right = Some(Box::new(left_right));
    }
    if let Some(r_right) = &mut root.right // Some(Box::new(right));
    {
        r_right.left = Some(Box::new(right_left));
        r_right.right = Some(Box::new(right_right));
    }
    print_preorder(&root);
}


fn today_tip() {
    println!("Today's tip: if let");
    let i = Some(1);
    if let Some(j) = &i {
        println!("{}", j);
    } 
    let i: Option<i32> = None;
    if let Some(j) = &i {
        println!("{}", j);
    } else {
        println!("None");
    }
    println!();
}
