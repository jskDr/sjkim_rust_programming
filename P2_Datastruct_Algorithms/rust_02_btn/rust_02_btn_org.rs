// Binary Tree Node
struct BTN {
    data: i32, // i8, i16, i32, i64, i128, isize
    left: Option<Box<BTN>>, // usize 
    right: Option<Box<BTN>>
}

fn print_inorder(root: &BTN) {
    if let Some(left) = &root.left {
        print_inorder(left);
    }
    println!("data: {}", root.data);
    if let Some(right) = &root.right {
        print_inorder(right);
    }
}

fn main() {
    let mut root = BTN {
        data: 0,
        left: None,
        right: None,
    };

    let left = BTN {
        data: 2,
        left: None,
        right: None,
    };

    let right = BTN {
        data: 3,
        left: None,
        right: None,
    };

    let left_left = BTN {
        data: 4,
        left: None,
        right: None,
    };

    let right_right = BTN {
        data: 5,
        left: None,
        right: None,
    };

    /*
         1
        / \
       2   3
      /     \ 
     4       5
    */
    // left: Option<Box<BTN>>
    // root->letf = left // Null
    root.data = 1;
    root.left = None; // Some(Box::new(left));
    root.right = Some(Box::new(right));
    // Option : None, Some(x)
    // root.left.as_mut().unwrap().left = Some(Box::new(left_left));
    if let Some(left) = &mut root.left {
        left.left = Some(Box::new(left_left));
    }
    root.right.as_mut().unwrap().right = Some(Box::new(right_right));
    print_inorder(&root);
}