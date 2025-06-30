use std::io;

fn main() {
    let x = 6;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x at the inner scope is : {x}");
    }
    println!("The value of x is : {x}");

    let bruh = -34 / 3;
    println!("Bruhh : {bruh}");

    let tup = (34, true, "scar");
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    let arr = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Not a valid index");

    let element = arr[index];

    println!("The number at the specified index is : {element}");

    another_function(5);

    let y = foo();
    println!("Fooo {y}");

    println!("{}", plus_one(3));
}

fn another_function(x: i32) {
    println!("Gimme {x} chocolates.");
}

fn foo() -> i32 {
    555
}

fn plus_one(mut x: i32) -> i32 {
    x = x + 1;
    return x;
}
