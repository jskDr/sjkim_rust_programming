fn check_subsum_rec(set: &Vec<u32>, sum: u32, n: usize) -> bool {
    if sum == 0 { true }
    else if n == 0 { false }
    else if set[n-1] > sum { check_subsum_rec(&set, sum, n-1) }
    else { check_subsum_rec(&set, sum, n-1) || check_subsum_rec(&set, sum - set[n-1], n-1)}
}

fn check_subsum_dp_tab(set: &Vec<u32>, sum: u32, n: usize) -> bool {
    // define 2-d array
    let mut tab: Vec<Vec<bool>> = vec![vec![false;sum as usize + 1];n+1];
    // sum = 3, n = 1 --> tab[2][4] : all false
    for i in 0..n + 1 {
        tab[i][0] = true; // tab[0][0] = tab[1][0] = true
    }
    for j in 1..sum as usize + 1 { // tab[0][1] = tab[0][2] = tab[0][3] = false
        // println!("j = {}", j);
        tab[0][j] = false; // n == 0 but sum(j) > 0 --> false
    }    
    // println!("tab = {:?}", tab);
    for i in 1..n+1 { 
        for j in 1..sum as usize + 1{
            if set[i-1] > j as u32 {
                tab[i][j] = tab[i-1][j];
            } else {
                tab[i][j] = tab[i-1][j] || tab[i-1][j - set[i-1] as usize];
            }
        }
    }
    tab[n][sum as usize]
  }

fn test_subsum(set: &Vec<u32>, sum: u32) {
    println!("Example: set = {:?} and sum = {}", set, sum);

    let result = check_subsum_rec(&set, sum, set.len());
    println!("Recursive method result -> {}", result);

    let result = check_subsum_dp_tab(&set, sum, set.len());
    println!("DP(Tabulation) result -> {}", result);
}

fn test_subsum_all_examples() {
    // subset sum problem
    let set: Vec<u32> = vec![3, 34, 4, 12, 5, 2]; 
    test_subsum(&set, 9);
    test_subsum(&set, 30);
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

fn test_dynamic_programming_fibonacci() {
    let n: u32 = 10;
    // tabulation method
    let result = fib_dp_tab(n);
    println!("fib_dp_tab({n}) = {}", result);

    // memoization method
    let mut fib_dp_memo: Fib = Fib::new(100);
    let result = fib_dp_memo.calc(n); // to give a value to object, define mut for a object 
    println!("fib_dp_memo.calc({n}) = {}", result);
}

fn main() {
    // subsum problem with both recursive and dynamic programming  
    println!("Check subset sum problem");
    test_subsum_all_examples();
    println!();

    // dynamic programming
    // fibonacci sequence
    println!("Find fibonacci sequence");
    test_dynamic_programming_fibonacci();
}
