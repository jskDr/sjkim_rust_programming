// Binary Tree Node
struct BTN {
    data: i32, // i8, i16, i32, i64, i128, isize
    left: Option<Box<BTN>>, // usize 
    right: Option<Box<BTN>>
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

    /*
         1
        / \
       2   3
    */
    
    // left: Option<Box<BTN>>
    // root->letf = left // Null
    root.left = Some(Box::new(left));
}