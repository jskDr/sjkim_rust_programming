fn optimized_bubble_sort(a: &mut Vec<u32>) {
    for i in 0..a.len() {
        for j in i+1..a.len() {
            if a[i] > a[j] {
                a.swap(i, j);
            }
        }
    }
}

fn selection_sort(a: &mut Vec<u32>) {
    let mut min_idx: usize;
    for i in 0..a.len() {
        min_idx = i;
        for j in i+1..a.len() {
            if a[j] < a[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            a.swap(i, min_idx)
        }
    }
}

fn insertion_sort(a: &mut Vec<u32>) { // [7, 8, 3, 4, 1, 2, 5, 6]
    for i in 1..a.len() { // i: 1, 2
        // reverse for loop with j=i..1, 2..1
        for j in (1..=i).rev() { // j: (1), 2 
            // compare a[i = j] and a[i-1 = j-1] 
            if a[j] < a[j-1] { // a[1]<a[0]: 8<7, a[2]<a[1]: 8<3, a[1]<a[0]: 3<7
                a.swap(j, j-1); // [7, 3, 8, 4, 1, 2, 5, 6], [3, 7, 8, 4, 1, 2, 5, 6]
            } else {
                break;
            }
        }
        // println!("{i} stage: {a:?}");
    }
}

fn pivot_idx(a: &mut Vec<u32>, st: usize, ed: usize) -> usize {
    let mut pv_idx: usize = st;
    let pv_val = a[pv_idx];
    for i in st+1..=ed {
        if a[i] < pv_val {
            // move j position to show the threthold for values lower than pv 
            pv_idx += 1;
            a.swap(i,pv_idx);
        }
        // println!("Step {i}: {a:?} with pivot index {pv_idx}");
    }
    a.swap(st,pv_idx);
    pv_idx
} 

fn pivot(a: &mut Vec<u32>) -> usize {
    let mut pv_idx: usize = 0;
    let pv_val = a[pv_idx];
    for i in 1..a.len() {
        if a[i] < pv_val {
            // move j position to show the threthold for values lower than pv 
            pv_idx += 1;
            a.swap(i,pv_idx);
        }
        // println!("Step {i}: {a:?} with pivot index {pv_idx}");
    }
    a.swap(0,pv_idx);
    pv_idx
} 

fn quicksort(a: &mut Vec<u32>, st: usize, ed: usize) {
    println!("st:{st}, ed:{ed}");
    if ed - st > 1 {
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
    let a_org: Vec<u32> = vec![7, 8, 3, 9, 4, 1, 0, 2, 5, 6];
    println!("Before Sort: {:?}", a_org);

    let mut a_sorted = a_org.clone();
    optimized_bubble_sort(&mut a_sorted);
    println!("After Buble Sort: {:?}", a_sorted);

    a_sorted = a_org.clone();
    selection_sort(&mut a_sorted);
    println!("After Selection Sort: {:?}", a_sorted);

    a_sorted = a_org.clone();
    insertion_sort(&mut a_sorted);
    println!("After Insertion Sort: {:?}", a_sorted);

    a_sorted = a_org.clone();
    let pv_idx = pivot(&mut a_sorted);
    println!("After Pivoting: {:?} with pv_idx = {}", a_sorted, pv_idx);

    a_sorted = a_org.clone();
    let st = 0;
    let ed = a_sorted.len()-1;
    let pv_idx = pivot_idx(&mut a_sorted, st,  ed);
    println!("After Pivoting with Index: {:?} with pv_idx = {}", a_sorted, pv_idx);

    a_sorted = a_org.clone();
    qsort(&mut a_sorted);
    println!("After Qsort: {:?}", a_sorted);

    println!("{:?}", (1..5).product::<i32>());
    println!("{:?}", (1..5).sum::<i32>());
}
