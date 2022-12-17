fn fib_rec(n: u32) -> u32 {
    0
}

fn fib_dp_tab(n: u32) -> u32 { // n: 10
    // tabulation dynamic programming for fibonacci sequence
    0
}

struct Fib {
    memo: Vec<u32>,
}

impl Fib {
    fn new(max_n: u32) -> Fib {
        Fib { memo: vec![0; max_n as usize] }
    }
    fn calc(&mut self, n: u32) -> u32 {
        0
    }    
}

fn main() {
    // get n from args[1] with default value 10
    let n = match std::env::args().nth(1) {
        Some(n) => n.parse::<u32>().unwrap(),
        None => 10,
    };
    // let n: u32 = 10;
    // recursive method
    let result = fib_rec(n);
    println!("fib_rec({n}) = {}", result);

    // tabulation method
    let result = fib_dp_tab(n);
    println!("fib_dp_tab({n}) = {}", result);

    // memoization method
    let mut fib_dp_memo: Fib = Fib::new(100);
    let result = fib_dp_memo.calc(n); // to give a value to object, define mut for a object 
    println!("fib_dp_memo.calc({n}) = {}", result);
}