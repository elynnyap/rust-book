fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // If we added a semicolon this would be a statement not an expression --> compile-time error
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}
