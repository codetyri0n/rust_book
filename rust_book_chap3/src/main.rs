use std::io;

fn main() {
    println!("Enter the temperature in celsius : ");
    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to get the celsius input");

    let number: f32 = celsius.trim().parse().expect("Please enter a valid number");

    let result = number * 9.0 / 5.0 + 32.0;

    println!("The result in fahrenheit is {result} F");
}
