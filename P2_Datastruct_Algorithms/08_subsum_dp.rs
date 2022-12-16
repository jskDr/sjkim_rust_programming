fn check_subsum(set: &[u32;6], sum: u32, n: usize) -> bool {
    if sum == 0 { true }
    else if n == 0 { false }
    else if set[n-1] > sum { check_subsum(&set, sum, n-1) }
    else { check_subsum(&set, sum, n-1) || check_subsum(&set, sum - set[n-1], n-1)}
}

fn test_subsum(set: &[u32;6], sum: u32) {
    let result = check_subsum(&set, sum, set.len());
    println!("test_subsum with sum = {:?} -> {}", set, result);
}

fn main() {
    let set: [u32;6] = [3_u32, 34, 4, 12, 5, 2]; 
    test_subsum(&set, 9);
    test_subsum(&set, 30);
    // test_subsum with sum = 9 -> true
    // test_subsum with sum = 30 -> false
}