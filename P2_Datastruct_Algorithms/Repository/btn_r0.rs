struct BinaryTreeNode {
    data: i32,
    left: Option<Box<BinaryTreeNode>>,
    right: Option<Box<BinaryTreeNode>>,
}

fn preorder(node: &Option<Box<BinaryTreeNode>>) {
    match node {
        None => return,
        Some(node) => {
            println!("{}", node.data);
            preorder(&node.left);
            preorder(&node.right);
        }
    }
}

fn main() {
    let root = BinaryTreeNode {
        data: 1,
        left: None,
        right: None,
    };

    let left = BinaryTreeNode {
        data: 2,
        left: None,
        right: None,
    };

    let right = BinaryTreeNode {
        data: 3,
        left: None,
        right: None,
    };

    let left_left = BinaryTreeNode {
        data: 4,
        left: None,
        right: None,
    };

    let left_right = BinaryTreeNode {
        data: 5,
        left: None,
        right: None,
    };

    let mut root_option_box = Some(Box::new(root));
    root_option_box.as_mut().unwrap().left = Some(Box::new(left));
    root_option_box.as_mut().unwrap().right = Some(Box::new(right));
    root_option_box.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(left_left));
    root_option_box.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(left_right));
    preorder(&root_option_box);
}