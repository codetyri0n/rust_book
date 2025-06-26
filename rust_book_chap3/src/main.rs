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
}
