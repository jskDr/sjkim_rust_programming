// Binary Tree Node
struct BTN {
    data: i32, // i8, i16, i32, i64, i128, isize
    left: Option<Box<BTN>>, // usize 
    right: Option<Box<BTN>>
}

// implementation for BTN
impl BTN {
}

fn print_preorder(root: &BTN) {
    let mut queue: VecDeque<&BTN> = VecDeque::new();
    println!();
}

fn main() {
    today_tip();

    println!("Preorder of BTN");
 
    /*
          1
        /   \
       2     3
      / \   /  \ 
     4   5 6    7
    */
  
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