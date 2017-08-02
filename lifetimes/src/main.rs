// all references in the signature must have the same lifetime 'a 
// the function will also return a string slice that is at least as long as the lifetime 'a
// any references whose lifetimes do not meet these constraints will violate the borrow checker
// lifetime annotations go into the function signature but NOT the body
// the generic lifetime 'a will be the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "hello";
    let y = String::from("world");
    println!("{}", longest(x, y.as_str()));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // create an instance of ImportantExcerpt struct that holds a reference 
    let i = ImportantExcerpt { part: first_sentence };
}

// if a struct holds a reference, its definition needs a lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str
}
