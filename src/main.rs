use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

// Every Function name with Snake case.
fn main() {
    // drawing_function()
    // ConstantValue_func();
    // funny_func();
    // mutable_value_func()
    // diff_with_mut_let();

    processingGuess();
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
fn funny_func() {
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6; Error

    let x = x + 1;

    let x = x + 2;
    println!("The value of x is {}", x);

    // The value of x is 5
    // The value of x is 8
}

fn mutable_value_func() {
    let mut x = 1;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value change to {}", x);

    const MAX_POINT: u32 = 100_000;
    println!("const practice {}", MAX_POINT);
}

fn diff_with_mut_let() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{}", spaces)

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // Error =>
}

fn processingGuess() {
    println!("Guess the number !");
    println!("Please input your guess");

    // Storing Values with Variablse with mutable variable that is currently bound to,
    // which is the result of calling String::new, a function that returns a new instance
    // of a String.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    println!("Your Number : {}", guess);

    let foo = 5; //immutable
    let mut bar = 5; //mutable

    // Mutable && Immutable variation !! Notes!!
}
