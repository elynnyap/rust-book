use std::cmp::PartialOrd;

// define a generic function that returns the largest element in a list 
// where the list can be composed of any type of elements
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    } 

    largest
}
    
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { // return a reference to the data in the field x
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T,U> Point2<T,U> {
    // Method can use different generic types than their struct's definition
    fn mixup<V,W>(self, other: Point2<V, W>) -> Point2<T, W> { 
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
    
    let p1 = Point2 { x: 10, y: 12 };
    let p2 = Point2 { x: "hello", y: 'c' };

    let p3 = p2.mixup(p1);
    println!("x: {}, y: {}", p3.x, p3.y);
}
