fn bubble_sort(a: &mut Vec<u32>) {
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

fn main() {
    println!("Sort Algoorithms");
    let a_org: Vec<u32> = vec![7, 8, 3, 4, 1, 2, 5, 6];
    println!("Before Sort: {:?}", a_org);

    let mut a_sorted = a_org.clone();
    bubble_sort(&mut a_sorted);
    println!("After Buble Sort: {:?}", a_sorted);

    let mut a_sorted = a_org.clone();
    selection_sort(&mut a_sorted);
    println!("After Selection Sort: {:?}", a_sorted);
}
