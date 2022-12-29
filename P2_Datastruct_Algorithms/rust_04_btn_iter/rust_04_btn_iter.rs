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
    // today_tip();

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
    if let Some(left) = &mut root.left {
        left.left = Some(Box::new(left_left));
        left.right = Some(Box::new(left_right));
    }
    if let Some(right) = &mut root.right {
        right.left = Some(Box::new(right_left));
        right.right = Some(Box::new(right_right));
    }
  
    print_preorder(&root);
}


