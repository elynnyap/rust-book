fn main() {
    let mut x = 5; // We must make x mutable in order to update its value; otherwise, compile-time error bc variables are immutable by default
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 1;
    println!("The value of y is: {}", y);
    let y = 2; // This new variable shadows the previous variable
    println!("The value of y is: {}", y); 
}
