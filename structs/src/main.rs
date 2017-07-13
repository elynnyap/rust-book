fn main() {
    // Declaring an instance of a struct
    let rect = Rectangle { length: 20, width: 50 };

    // Calling a method on a struct
    let area = rect.area();

    // By deriving the Debug trait for our struct we can print it out
    println!("The area of {:?} is {}", rect, area);

    let rect2 = Rectangle { length: 10, width: 20 };
    let rect3 = Rectangle { length: 15, width: 60 };

    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
    
    println!("Here's a square: {:?}", Rectangle::square(10));
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {

    // A struct's method doesn't necessarily need to have &self as the first param. Call this using ::
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }
   
    // Can add more params to struct method, in addition to &self
    // We want to borrow a reference instead of transferring ownership to the method 
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.length <= self.length && other_rect.width <= self.width
    }
}
