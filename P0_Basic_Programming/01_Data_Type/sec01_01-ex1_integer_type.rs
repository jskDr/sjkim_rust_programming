fn eg_variable_type_define() 
{
    // integer type: i8, i16, i32, i64, i128, isize
    let i: i32 = 1;
    // usigned integer type: u8, u16, u32, u64, u128, usize
    let ui: u32 = 0;
    // char type: char
    let c: char = 'a';
    // floating point type: f32, f64
    let df: f64 = 3.14;

    println!("i = {}, ui = {}, c = {}, df = {}", i, ui, c, df);
}

fn eg_type_converting_by_as() {
    let i: i32 = 1;
    let ui: u32 = i as u32;
    println!("i = {}, ui = {}", i, ui);
}

fn option_type() {
    let a: Option<i32>;
    a = None;
    match a {
        Some(x) => println!("a = Some({})", x),
        None => println!("a = None"),
    }

    let b = Some(1);
    match b {
        Some(x) => println!("b = Some({})", x),
        None => println!("b = None"),
    }

    let c = Some(1);
    match c {
        Some(x) => {
            println!("b = Some({})", x);
            println!("c.unwrap - 1 = {}", c.unwrap() - 1);
        }
        None => println!("b = None"),
    }
}

fn box_type() {
    let a = Box::new(1);
    let b = 2;
    let c = *a + b;
    println!("a = {}", a); // println solves unwrap of Box automatically
    println!("c = {}", c);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn box_type() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn array_type()

fn main() {
    eg_variable_type_define();
    println!();

    eg_type_converting_by_as();
    println!();

    println!("Option type:");
    option_type();
    println!();

    println!("Box type:");
    box_type();
    println!();

    println!("Array type:");
    array_type();
    println!();
}