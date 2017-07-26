use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    // Rust provides a vec macro that we can use to create a Vector
    // No need for explicit type annotation since Rust can infer from the values we provided
    let v = vec![1,2,3];

    // OR we can declare a vector like so:
    let mut v2 = Vec::new(); // note that v2 must be mutable for us to change its values

    v2.push(1);
    v2.push(2);
    v2.push(3);

    // we can access a value in a vector like so:
    let third = &v[2];
    print!("Third element is {}\n", third);

    // but if a non-existent element is referenced, Rust will cause a panic!
    // if accessing an element beyond the valid range would happen occasionally under normal
    // circumstances then capture the element in an Option<&T>
    let fourth = v.get(3);
    let first = v.get(0);

    print_num(fourth);
    print_num(first);
    
    // if we have an immutable borrow of v2, then we cannot also have a mutable borrow before the
    // first borrow goes out of scope. this prevents dangling references.
    let x = &v2[0];
    //v2.push(4);

    // if we want to have a vec that effectively contains multiple types, but we know what that finite set of
    // types look like, we can use Enum
    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::String(String::from("hello"))
    ];

    /* STRINGS */

    let mut s = String::new();
    let data = "initial contents"; // data is of type &str; string literals are string slices

    // create a string containing "initial contents"
    let s = data.to_string();
    let s = "initial contents".to_string(); // to_string method works on a string literal directly 
    let s = String::from("initial contents"); // String::from() and to_string() do the exact same thing

    // append to a string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // push_str takes a string slice
    println!("{}", &s1);

    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("{}", &s1);

    let string_a = String::from("tic");
    let string_b = String::from("tac");
    let string_c = String::from("toe");

    let abc = format!("{}-{}-{}", string_a, string_b, string_c); // using the format macros does not take ownership of any param
    println!("{}", abc);
    let abc = string_a + "-" + &string_b + "-" + &string_c; // this uses add() which takes ownership of string_a
    println!("{}", abc);

    /* HASH MAPS */

    // all values must be of the same type and all keys must be of the same type
    let mut scores = HashMap::new();
    scores.insert(String::from("red"), 10);
    scores.insert(String::from("blue"), 90);

    // we can use collect() on two vectors to create a hashmap
    let teams = vec![String::from("red"), String::from("blue")];
    let points = vec![10,90];
    let scores: HashMap<_,_>  = teams.iter().zip(points.iter()).collect(); // need to add type annotation bc collect() gathers
    // data up into several different collection types.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Black");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // the bindings field_name and field_value are no longer valid as they have been moved into the
    // hashmap with insert()
    // if we insert references into the hashmap then the values will not be moved - but the values
    // that the references point to must be valid for as long as the hash map is valid.
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    String(String)
}

fn print_num(x: Option<&i32>) {
    match x {
        None => println!("No value."),
        Some(i) => println!("Value is {}", i)
    }
}
