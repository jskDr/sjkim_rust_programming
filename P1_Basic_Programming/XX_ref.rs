/*
fn ex1_bad() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];
    // For for loop, the values is moved, so it can't be used again.
    // Hence, values should be borrowed, such as &values.
    for value in values {
        print!("{} ", value);
    }
    println!();

    for value in values {
        print!("{} ", value);
    }
    println!();
}
*/


fn ex1_good() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];    
    for value in &values {
        print!("{} ", value);
    }
    println!();
    for value in values {
        print!("{} ", value);
    }
    println!();
}

fn for_prt(values: &Vec<i32>) {
    // values is borrowed, so &values is not needed.
    for value in values { 
        print!("{} ", value);
    }
    println!();
    // values is borrowed, so &values is not needed all the time.
    for value in values {
        print!("{} ", value);
    }
    println!();
}

fn ex2_good() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];
    for_prt(&values);
    for_prt(&values);
}

fn ex3_good() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("values[1] = {}", values[1]);
    let values = &values;
    println!("values[1] = {}", values[1]);
    println!("values = {:?}", *values);

    for value in values {
        print!("{} ", value);
    }
    println!();
    for value in values {
        print!("{} ", value);
    }
    println!();
}

fn ex4_bad() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];
    let values = &values;
    // For for loop, the values is moved, so it can't be used again.
    // Hence, values should be borrowed, such as &values.
    // for value in *values {
    for value in &*values {
        print!("{} ", value);
    }
    println!();
    /*
    for value in values {
        print!("{} ", value);
    }
    println!();
    */
}

fn main() {
    ex1_good();   
    ex2_good(); 
    ex3_good();
    
    ex4_bad();
}