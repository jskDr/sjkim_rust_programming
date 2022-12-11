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
    let mut stack: Vec<&BTN> = Vec::new();
    let mut curr = root; 
    let mut curr_flag = true;
    let mut non_empty_flag = true;
    while curr_flag || non_empty_flag {
        while curr_flag { 
            print!("{} ", curr.data);
            stack.push(curr);
            if let Some(left) = &curr.left {
                curr = left;
            }
            else {
                break;
            }
        }
        if let Some(root) = stack.pop() { // stack.pop() is always Option type
            if let Some(right) = &root.right {
                curr = right;
                curr_flag = true;
            } else {
                curr_flag = false;
            }    
            non_empty_flag = true;
        } else {
            non_empty_flag = false;
        } 
    }
    println!();
}

fn print_preorder2(root: &BTN) {
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        print!("{} ", node.data);
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
  
    // print_preorder2(&root);
    print_preorder(&root);
}


