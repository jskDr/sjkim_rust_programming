// #[warn(dead_code)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn gen_nodes(n_nodes: i32) -> Option<Box<Node>> {
    let mut post_node: Option<Box<Node>> = None;
    // None <- 0 <- 1 <- 2 <- 3 <- 4 <- 5 <- 6 <- 7 <- 8 <- 9 <- 10
    for i in 0..n_nodes {
        let node_base = Node {
            data: i,
            next: post_node,
        };
        post_node = Some(Box::new(node_base));
    }
    post_node
}

fn traverse_nodes(anode: &Option<Box<Node>>) {
    let mut node = anode;
    while let Some(n) = node {
        let unbox_n = &*n; //&*n; 
        println!("{}", unbox_n.data);
        node = &unbox_n.next;
    }
}

fn main() {
    println!("Linked List");
    let post_node: Option<Box<Node>> = gen_nodes(5); 
    //let anode = &*post_node.as_ref().unwrap();
    //println!("d.data = {}", anode.data);
    traverse_nodes(&post_node);

    let x_opt: Option<isize> = Some(10);
    let x = x_opt.unwrap();
    println!("x = {}", x);

    let x_opt: Option<Box<isize>> = Some(Box::new(10));
    let x = x_opt.unwrap();
    println!("x = {}", *x);

    let x_opt = Some(Box::new(10));
    let x = x_opt.unwrap();
    println!("x = {}", *x);

    let x_opt = Some(Box::new(Node {
        data: 10,
        next: None,
    }));
    let x_box = x_opt.unwrap();
    let x_unbox = *x_box;
    let x = x_unbox.data;
    println!("x = {}", x);

    let x_opt = Some(Box::new(Node {
        data: 10,
        next: None,
    }));
    let x_box = x_opt.unwrap();
    let x_unbox = *x_box;
    let x = x_unbox.data;
    println!("x = {x}");
    println!("x = {}", x_unbox.data);

    println!("Linked List");
    let x_opt: Option<Box<Node>> = gen_nodes(5); 
    let x_box = x_opt.as_ref().unwrap();
    let x_unbox = &*x_box;
    let x = x_unbox.data;
    println!("x = {x}");
    println!("x = {}", x_unbox.data); 

    let x_box = x_opt.unwrap();
    let x_unbox = *x_box;
    let x = x_unbox.data;
    println!("x = {x}");
    println!("x = {}", x_unbox.data);     
}
