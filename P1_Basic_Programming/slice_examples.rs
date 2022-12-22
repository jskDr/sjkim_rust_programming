

fn ex1() {
    // Referencing and mutability are not the samething though they looks confusingly similar
    let original = [1,2,3,4,5];
    let slice = &original[1..3];
    // slice[0] = 7; // error[E0596]: cannot borrow `*slice` as mutable, as it is behind a `&` reference
    println!("original: {original:?}");
    println!("slice: {slice:?}");
}

fn ex2() {
    // let original = [1,2,3,4,5];
    let mut original = [1,2,3,4,5];
    let slice = &mut original[1..3]; // error[E0596]: cannot borrow `original` as mutable, as it is not declared as mutable
    slice[0] = 7;
    // println!("original: {original:?}"); // error[E0502]: cannot borrow `original` as immutable because it is also borrowed as mutable 
    println!("slice: {slice:?}");
    println!("original: {:?}", original);
    // original[0] = 0;
    // println!("original: {:?}", original);
}

fn ex3() {
    let slice = &[1,2,3,4,5];
    for elem in slice {
        print!("{} ", elem);
    }
    println!();
}

fn ex4() {
    let mut slice = [5, 1, 3, 2, 4];
    // slice.sort(); // error[E0596]: cannot borrow `slice` as mutable, as it is not declared as mutable
    slice.sort();
    println!("slice: {:?}", slice);
}

fn ex5() {
    let slice = [1, 2, 3, 4, 5];
    let index = slice.binary_search(&3);
    let index = match index {
        Ok(i) => i,
        Err(err) => {
            println!("Not found: {:?}", err);
            return;
        }
    };
    println!("index: {:?}", index);
}

fn prt(a: &mut [u32]) {
    a[0] = 100;
    println!("{a:?}");
}

fn ex6() {
    let mut a = vec![1, 2, 3, 4, 5];
    let slice = &mut a[1..3];
    println!("{:?}", slice);
    prt(slice);
    let slice2 = &mut a[2..4];
    println!("{:?}", slice2);
    prt(slice2);
}

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
}