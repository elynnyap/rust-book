extern crate traits;
use traits::Summarizable; // must bring Summarizable trait into scope to use it
use traits::notify;

fn main() {
    let tweet = traits::Tweet {
        username: String::from("elynn"),
        content: String::from("hello world"),
        reply: String::from("tweet"),
        retweet: String::from("retweet")
    };
    println!("Tweet: {}", tweet.summary());
    notify(tweet);
}
