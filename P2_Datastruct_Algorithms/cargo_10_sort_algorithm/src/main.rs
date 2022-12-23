fn optimized_bubble_sort(a: &mut Vec<u32>) {
}

fn pivot_idx(a: &mut Vec<u32>, st: usize, ed: usize) -> usize {
    0
} 

fn quicksort(a: &mut Vec<u32>, st: usize, ed: usize) {
}

fn qsort(a: &mut Vec<u32>) {
    // st: usize, ed: usize
    quicksort(a, 0, a.len()-1);
}

fn main() {
    println!("Sort Algoorithms");
    let a_org: Vec<u32> = vec![7, 8, 3, 9, 4, 1, 0, 2, 5, 6];
    println!("Before Sort: {:?}", a_org);

    let mut a_sorted = a_org.clone();
    optimized_bubble_sort(&mut a_sorted);
    println!("After Buble Sort: {:?}", a_sorted);

    a_sorted = a_org.clone();
    let st = 0;
    let ed = a_sorted.len()-1;
    let pv_idx = pivot_idx(&mut a_sorted, st,  ed);
    println!("After Pivoting with Index: {:?} with pv_idx = {}", a_sorted, pv_idx);

    a_sorted = a_org.clone();
    qsort(&mut a_sorted);
    println!("After Qsort: {:?}", a_sorted);
}
