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

fn print_listorder(root: &BTN) {
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

fn print_inorder(root: &BTN) {
    let mut stack: Vec<&BTN> = Vec::new();
    let mut curr = root;
    let mut curr_flag = true;
    let mut non_empty_stack_flag = false;
    while curr_flag || non_empty_stack_flag {
        while curr_flag {
            stack.push(curr); // (1, 2, 4), (1, 5), (3, 7), (8)
            if let Some(left) = &curr.left {
                curr = left; // 2, 4, 7
                // curr_flag = true;
            }
            else {
                curr_flag = false;
            }
        }
        if let Some(root) = stack.pop() { // root: 4, 2, 5, 1, 7, 3, 8
            print!("{} ", root.data); // 4, 2, 5, 1, 7, 3, 8 
            if let Some(right) = &root.right { // 5, 3, 8 
                curr = right; // 5, 3, 8
                curr_flag = true;
            } else {
                curr_flag = false;
            } // curr_flag = false
            non_empty_stack_flag = true;
        } else {
            non_empty_stack_flag = false;
        }        
    }
    println!();
}

fn print_postorder(root: &BTN) {}


fn main() {
     /*
          1
        /   \
       2     3
      / \   /  \ 
     4   5 6    7
    */
 
    let mut root = BTN::new(1);
    root.left = Some(Box::new(BTN::new(2)));
    root.right = Some(Box::new(BTN::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(BTN::new(4)));
    root.left.as_mut().unwrap().right = Some(Box::new(BTN::new(5)));
    root.right.as_mut().unwrap().left = Some(Box::new(BTN::new(6)));
    root.right.as_mut().unwrap().right = Some(Box::new(BTN::new(7)));
   
    print_listorder(&root);
    print_preorder(&root);
    print_inorder(&root);
    print_postorder(&root);
}


