struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

fn main() {
    let a = ListNode {
        data: 3.4,
        next: None,
    }; 
    let b = ListNode {
        data: 5.6,
        next: Some(Box::new(a)),
    };
    let c = ListNode {
        data: 7.8,
        next: Some(Box::new(b)),
    };
    println!("c.data = {}", c.data);
    println!("c.next.data = {}", c.next.as_ref().unwrap().data);
    println!("c.next.next.data = {}", c.next.as_ref().unwrap().next.as_ref().unwrap().data);

}
