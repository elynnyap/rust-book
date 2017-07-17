#![allow(unused_variables)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Connecticut,
    NewYork,
    Oregon,
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            //match arm code can contain multiple lines as long as it's an expression (returns a value)
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            //bind pattern to a value to be used in match arm code
            println!("Quarter from {:?}", state);
            25
        }
    }
}

//using match for extracting value from Option if it's present
fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1
    }
    //match must be EXHAUSTIVE, e.g. compiler will throw error if we don't handle None case
}

fn main() {
    println!("Value of a nickel is {}", value_in_cents(Coin::Nickel));
    println!("Value of a quarter is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Value of a penny is {}", value_in_cents(Coin::Penny));

    println!("Two plus one is {}", plus_one(Some(2)));
    println!("None plus one is {}", plus_one(None));
    
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three!");
    } else {
        println!("not three!");
    }
    //the above if-let-else block is the same as the following match block
    match some_value {
        Some(3) => println!("three!"),
        _ => println!("not three!"),
    }
}
