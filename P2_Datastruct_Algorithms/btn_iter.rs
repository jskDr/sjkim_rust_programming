// import vecdeque
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
    queue.push_back(root); // &BTN(1, None, None)
    while !queue.is_empty() { // 1,
        let mut node = queue.pop_front().unwrap(); // node = BTN(1, S(B(2)), S(B(3)))
        print!("{} ", node.data); // 1,        
        while let Some(left) = &node.left { // left = &B(2,3,4), &B(3,4,5)
            queue.push_back(left); // &B(2,3,4)
            node = left.as_ref();  // node = &B(2,3,4)
            println!("{} ", node.data); // 2,
            println!("{} ", node.left.as_ref().unwrap().data); // 3,
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


fn _today_tip() {
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