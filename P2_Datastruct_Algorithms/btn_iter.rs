// Binary Tree Node
struct BTN {
    data: i32, // i8, i16, i32, i64, i128, isize
    left: Option<Box<BTN>>, // usize 
    right: Option<Box<BTN>>
}

fn print_inorder(root: &BTN) {
    // define a queue
    let mut queue = Vec::new();
    queue.push(root);
    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        println!("data: {}", node.data);
        if let Some(left) = &node.left {
            queue.push(left);
        }
        if let Some(right) = &node.right {
            queue.push(right);
        }
    }
}

fn main() {
    let mut root = BTN {
        data: 1,
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
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    // if let Some(left) = &mut root.left {
    //     left.left = Some(Box::new(left_left));
    // }
    // root.right.as_mut().unwrap().right = Some(Box::new(right_right));
    // let root = Some(Box::new(root));
    print_inorder(&root);
}