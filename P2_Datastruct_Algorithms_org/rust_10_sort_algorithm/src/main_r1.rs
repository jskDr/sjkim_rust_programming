fn optimized_bubble_sort(a: &mut Vec<u32>) {
    for i in 0..a.len() {
        for j in i+1..a.len() {
            if a[j] < a[i] {
                a.swap(j, i)
            } 
        }
    }
}

fn pivot_idx(a: &mut Vec<u32>, st: usize, ed: usize) -> usize {
    // depart larger values to the right of the pivot
    let mut pv_idx = st;
    let pv_val = a[pv_idx];
    for i in st+1..=ed {
        if a[i] < pv_val {
            pv_idx += 1;
            a.swap(i, pv_idx);
        }
    }
    a.swap(st, pv_idx);
    pv_idx
} 

fn quicksort(a: &mut Vec<u32>, st: usize, ed: usize) {
    if ed - st > 0 {
        // ed - st = 1 means two elements as =ed is inclusive approach here
        let pv = pivot_idx(a, st, ed);
        if pv > st { quicksort(a, st, pv - 1)}
        if pv < ed { quicksort(a, pv + 1, ed);}
    }
}

fn qsort(a: &mut Vec<u32>) {
    // st: usize, ed: usize
    quicksort(a, 0, a.len()-1);
}

fn main() {
    println!("Sort Algoorithms");
    let a_org: Vec<u32> = vec![7, 8, 3, 7, 4, 1, 0, 2, 5, 6];
    println!("Before Sort: {:?}", a_org);

    let mut a_sorted = a_org.clone();
    optimized_bubble_sort(&mut a_sorted);
    println!("After Buble Sort: {:?}", a_sorted);

    a_sorted = a_org.clone();
    let st = 0;
    let ed = a_sorted.len()-1;
    let pv_idx = pivot_idx(&mut a_sorted, st, ed);
    println!("After Pivoting with Index: {:?} with pv_idx = {}", a_sorted, pv_idx);

    a_sorted = a_org.clone();
    qsort(&mut a_sorted);
    println!("After Qsort: {:?}", a_sorted);
}
