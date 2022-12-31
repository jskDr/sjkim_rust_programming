
fn first_repeated_ch(s: &str) -> Option<char> {
    // define 256 zero elements array
    let mut hash_tbl: [u32; 256] = [0; 256];
    for c in s.chars() {
        if hash_tbl[c as usize] == 1 {
            return Some(c);
        } else {
            hash_tbl[c as usize] = 1;
        }
    }
    None
}


fn run_first_repeated_ch(s: &str) {
    match first_repeated_ch(s) {
        Some(c) => println!("The first repeated character is {c} in {s}."),
        None => println!("No repeated character is in {s}.")
    }
}

fn main() {
    let s = "abca";
    run_first_repeated_ch(s);
    let s = "abc";
    run_first_repeated_ch(s);
}
