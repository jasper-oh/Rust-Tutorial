use ferris_says::say;
use std::io::{stdout, BufWriter};

// Every Function name with Snake case.
fn main() {
    // drawing_function()
    // ConstantValue_func();
    MutableValue_func()
}

fn drawing_function() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
fn constant_value_func() {
    let x = 5;
    println!("The value of x is {}", x);

    // x = 6;
    // println!("The value of x is {}",x);
    // error message let would be use for constant value
    // But, when you choose to use variation Mutable variation then,
}
fn mutable_value_func() {
    let mut x = 1;
    println!("The value of x is {}", x)
}
