// #[warn(dead_code)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn gen_nodes(N: i32) -> Option<Box<Node>> {
    let mut post_node: Option<Box<Node>> = None;
    // None <- 0 <- 1 <- 2 <- 3 <- 4 <- 5 <- 6 <- 7 <- 8 <- 9 <- 10
    for i in 0..N {
        let node_base = Node {
            data: i,
            next: post_node,
        };
        post_node = Some(Box::new(node_base));
    }
    post_node
}

fn main() {
    println!("Linked List");
    let mut post_node: Option<Box<Node>> = None;
    // None <- 0 <- 1 <- 2 <- 3 <- 4 <- 5 <- 6 <- 7 <- 8 <- 9 <- 10
    for i in 0..10 {
        let node_base = Node {
            data: i,
            next: post_node,
        };
        post_node = Some(Box::new(node_base));
    }
    let anode = *post_node.unwrap();
    println!("d.data = {}", anode.data);
}