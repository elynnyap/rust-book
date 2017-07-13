fn main() {
    let s1 = String::from("hello world world world");
   
    // Pass a slice as argument so that function can handle more cases 
    let first_word = get_first_word(&s1[..]);

    println!("first word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // To iterate over the string it must be read as bytes

    // iter() returns each element in a collection
    // enumerate() wraps the result of iter() and returns each element as part of a tuple
    for (i, &item) in bytes.iter().enumerate() { // pattern matching!!
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
