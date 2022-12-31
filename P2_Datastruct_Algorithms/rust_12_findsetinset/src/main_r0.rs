use std::collections::HashMap;

fn binary_search(vec: &Vec<i32>, st: usize, ed: usize, val: i32) -> bool {
    // v1: vec![3, 7, 2, 9, 5]; v2: vec![7, 9, 8]
    let mid = (st+ed) / 2;
    if val == vec[mid] {
        true
    } else if val < vec[mid] && st + 1 <= mid {
        binary_search(vec, st, mid - 1, val)
    } else if val > vec[mid] && mid + 1 <= ed {
        binary_search(vec, mid + 1, ed, val)
    } else {
        false
    }
}

fn findsetinset_basic(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    for val in v2 { // v1: vec![3, 7, 2, 9, 5]; v2: vec![7, 9, 8]
        let mut i = 0;
        while i < v1.len() {
            if *val == v1[i] {
                break;
            }
            i += 1;
        }
        if i == v1.len() {
            return false;
        }
    }
    true
}

fn findsetinset_bs(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let mut v1 = v1.clone();
    v1.sort();
    for val in v2 {
        if !binary_search(&v1, 0, v1.len()-1, *val) {
            return false
        }
    }
    true
}

fn findsetinset_hash(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let mut v2_map = HashMap::<i32,i32>::new(); 
    for val in v2 {
        v2_map.insert(*val, 1);
    }
    for val in v1 {
        if v2_map.contains_key(val) {
            v2_map.remove(val);
        }
    }
    v2_map.len() == 0
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