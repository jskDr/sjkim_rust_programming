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

fn main() {
    eg_variable_type_define();
    eg_type_converting_by_as();
}