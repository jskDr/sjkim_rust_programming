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

fn main() {
    println!("Sort Algoorithms");
    let a_org: Vec<u32> = vec![7, 8, 3, 4, 1, 2, 5, 6];
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

    println!("{:?}", (1..5).product::<i32>());
}
