fn today_tip() {
    println!("Today's tip: if let");
    let i = Some(1);
    if let Some(j) = &i {
        println!("{}", j);
    } 
    let i: Option<i32> = None;
    if let Some(j) = &i {
        println!("{}", j);
    } else {
        println!("None");
    }
    println!();
}

fn main() {
    today_tip();
}