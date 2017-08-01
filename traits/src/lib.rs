// definition of a Summarizable trait
// we are allowed to implement a trait on a type as long as either the trait or the type is
// internal to our crate.
pub trait Summarizable {
    // method signatures that types implemeting this trait need to have
    // each type that implements this trait will have to provide its own custom behavior for the
    // body of the method.
    fn summary(&self) -> String;

    // or we can also have a default implementation of a method
    /*fn summary(&self) -> String {*/
        //String::from("Read more...")
    /*}*/
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implement the Summarizable trait for the NewsArticle type
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        // fill the method body with the specific behavior that we want the methods of the trait to
        // have for this type.
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// use trait bounds to specify that item must be of a type that implements the Summarizable trait.
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
