fn check_subsum_rec(set: &Vec<u32>, sum: u32, n: usize) -> bool {
    true
}

fn check_subsum_dp_tab(set: &Vec<u32>, sum: u32, n: usize) -> bool {
    true
  }

fn test_subsum(set: &Vec<u32>, sum: u32) {
    println!("Example: set = {:?} and sum = {}", set, sum);

    let result = check_subsum_rec(&set, sum, set.len());
    println!("Recursive method result -> {}", result);

    let result = check_subsum_dp_tab(&set, sum, set.len());
    println!("DP(Tabulation) result -> {}", result);
}

fn main() {
    // subset sum problem
    let set: Vec<u32> = vec![3, 34, 4, 12, 5, 2]; 
    test_subsum(&set, 9);
    test_subsum(&set, 30);
}