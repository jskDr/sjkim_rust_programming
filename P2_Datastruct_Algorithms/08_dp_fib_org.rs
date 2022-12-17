fn fib_rec(n: u32) -> u32 {
    if n == 0 {0}
    else if n == 1 {1}
    else {fib_rec(n-1) + fib_rec(n-2)}
}

fn fib_dp_tab(n: u32) -> u32 { // n: 10
    // tabulation dynamic programming for fibonacci sequence
    if n == 0 { return 0; }
    let mut tab = vec![0; n as usize + 1]; // tab[11]
    tab[0] = 0;
    tab[1] = 1;
    for i in 2..n as usize + 1 { // 2, 3, .. n
        tab[i] = tab[i-1] + tab[i-2];
    }
    tab[n as usize]
}

struct Fib {
    memo: Vec<u32>,
}

impl Fib {
    fn new(max_n: u32) -> Fib {
        Fib { memo: vec![0; max_n as usize] }
    }
    fn calc(&mut self, n: u32) -> u32 {
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        if self.memo[n as usize] != 0 { return self.memo[n as usize]; }
        self.memo[n as usize] = self.calc(n-1) + self.calc(n-2);
        self.memo[n as usize]
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