fn main() {
    let x = 6;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x at the inner scope is : {x}");
    }
    println!("The value of x is : {x}");
}
