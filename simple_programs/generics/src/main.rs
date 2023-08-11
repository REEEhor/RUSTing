#[allow(unused_imports)]
use generics::{Article, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("REEEhor"),
        content: String::from("Ice-cream may be a soup."),
        reply: false,
        retweet: false,
    };

    println!("New tweet: {}", tweet.summarize());
}
