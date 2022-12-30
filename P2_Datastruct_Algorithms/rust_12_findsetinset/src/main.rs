use std::collections::HashMap;

fn binary_search(vec: &Vec<i32>, st: usize, ed: usize, val: i32) -> bool {
}

fn findsetinset_bs(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    return true;
}

fn findsetinset_basic(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    true
}

fn findsetinset_hash(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    true
}

fn run(v1: Vec<i32>, v2: Vec<i32>, exact_result: bool) {
    println!("");
    println!("v1 = {v1:?}, v2 = {v2:?}");
    
    let result = findsetinset_basic(&v1, &v2); // O(MN)
    println!("Result with basic search: {result}");

    let result = findsetinset_bs(&v1, &v2); // O(MlogN)
    println!("Result with binary search: {result}");

    let result = findsetinset_hash(&v1, &v2); // O(M+N)
    println!("Result with Hashmap: {result}");

    println!("Check if result must be {exact_result}.");    
}

fn main() {
    let v1: Vec<i32> = vec![3, 7, 2, 9, 5];
    let v2: Vec<i32> = vec![7, 9, 8];
    run(v1, v2, false);

    let v1 = vec![3, 7, 2, 9, 5];
    let v2 = vec![7, 9, 5];
    run(v1, v2, true);
}