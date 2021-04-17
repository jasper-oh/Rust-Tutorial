use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    drawing_function()
}

fn drawing_function() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
fn guessing_func() {
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}",x);
    // error message let would be use for constant value
}
