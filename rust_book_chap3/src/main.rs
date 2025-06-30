use std::io;

fn main() {
    println!("Enter the number for calculation :");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Enter valid input");

    let input: i32 = a.trim().parse().expect("Enter a valid number");

    println!(
        "The Fibonacci computation for {} is: {}",
        input,
        fibonacci(input)
    );
}

fn fibonacci(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
