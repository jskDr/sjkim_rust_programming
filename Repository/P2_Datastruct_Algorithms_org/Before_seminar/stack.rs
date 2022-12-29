// define a struct
struct Stack {
    capacity: i32,
    top: i32,
    data: Vec<i32>
}

// implement the struct
impl Stack {
    // constructor
    fn new(capacity: i32) -> Stack {
        Stack {
            top: -1,
            capacity: capacity,
            data: vec![0; capacity as usize]
        }
    }
    fn is_full(&self) -> bool {
        self.top == self.capacity - 1
    }
    fn push(&mut self, item: i32) {
        if self.is_full() {
            println!("Stack Overflow");
            return;
        }
        self.top += 1;
        self.data[self.top as usize] = item;
    }
    fn is_empty(&self) -> bool {
        self.top == -1
    }
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            println!("Stack Underflow");
            return -1;
        }
        let item = self.data[self.top as usize];
        self.data[self.top as usize] = 0;
        self.top -= 1;
        item
    }   
}

fn println_local(s: &Stack) {
    println!("top: {}, capacity: {}, data: {:?}", s.top, s.capacity, s.data);
}

fn main() {
    let mut s = Stack::new(10);
    println_local(&s); // bring the struct into the function
    s.push(1);
    s.push(2);
    println_local(&s);
    println!("pop: {}", s.pop());
    println!("pop: {}", s.pop());
    println_local(&s);
}