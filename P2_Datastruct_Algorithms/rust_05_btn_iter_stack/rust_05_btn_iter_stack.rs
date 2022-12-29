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

fn print_levelorder(root: &BTN) {
    let mut queue: VecDeque<&BTN> = VecDeque::new();
    queue.push_back(root); // 1
    while !queue.is_empty() { // True
        // queue: (2, 3) -> (3, 4, 5)
        // stack, queue produce Option type
        let node = queue.pop_front().unwrap(); // 1
        print!("{} ", node.data);
        if let Some(left) = &node.left {
            queue.push_back(left); // 2, 4
        }
        if let Some(right) = &node.right {
            queue.push_back(right); // 3, 5
        }
    } 
    println!();
}

fn print_preorder(root: &BTN) {
    // use stack
    let mut stack: Vec<&BTN> = Vec::new();
    // let mut queue: VecDeque<&BTN> = VecDeque::new();
    stack.push(root); 
    while !stack.is_empty() { 
        let node = stack.pop().unwrap(); 
        print!("{} ", node.data); // 1 -> (3,2), 2 -> (3,5,4)  
        if let Some(right) = &node.right {
            stack.push(right); 
        }
        if let Some(left) = &node.left {
            stack.push(left);
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
  
    println!("Levelorder traversal of BTN");
    print_levelorder(&root);
    println!("Preorder traversal of BTN");
    print_preorder(&root);
}


