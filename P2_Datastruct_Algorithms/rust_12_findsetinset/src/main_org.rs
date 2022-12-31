use std::collections::HashMap;

fn binary_search(vec: &Vec<i32>, st: usize, ed: usize, val: i32) -> bool {
    // [2, 3, 5, 7, 9] and 3, [2,3] and 3
    let mid = (st+ed) / 2; // (0+4)/2 = 2, 0
    if val == vec[mid] { // 3 == 5, no, 3 == 2, no
        true
    } else if val < vec[mid] && st + 1 <= mid { // 3 < 5 && 1 <= 2
        binary_search(vec, st, mid - 1, val)
    } else if val > vec[mid] && mid + 1 <= ed { // 3 > 2 && 1 <= 1
        binary_search(vec, mid + 1, ed, val)
    } 
    else {
        false
    }
}

fn findsetinset_bs(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let mut v1c = v1.clone();
    v1c.sort(); // reference function?
    for val in v2 {
        if !binary_search(&v1c, 0, v1c.len()-1, *val) {
            return false
        }
    }
    return true;
}

fn findsetinset_basic(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    for val in v2 {
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

fn findsetinset_hash(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let mut v2_map = HashMap::new();
    for val in v2 {
        v2_map.insert(val, 1);
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
    
    let result = findsetinset_basic(&v1, &v2);
    println!("Result with basic search: {result}");

    let result = findsetinset_bs(&v1, &v2);
    println!("Result with binary search: {result}");

    let result = findsetinset_hash(&v1, &v2);
    println!("Result with Hashmap: {result}");

    println!("Check if result must be {exact_result}.");    
}

fn main() {
    let v1 = vec![3, 7, 2, 9, 5];
    let v2 = vec![7, 9, 8];
    run(v1, v2, false);

    let v1 = vec![3, 7, 2, 9, 5];
    let v2 = vec![7, 9, 5];
    run(v1, v2, true);
}