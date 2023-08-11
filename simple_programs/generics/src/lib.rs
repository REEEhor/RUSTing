pub trait Summary {
    fn summarize(&self) -> String;
}

/*
 * It is not possible to return multiple different types
fn some_summarizable(want_tweet: bool) -> Box<impl Summary> {
    let article = Box::new(Article {
        headline: String::from("no_headline"),
        author: String::from("no_author"),
        content: String::from("_"),
    });

    let tweet = Box::new(Tweet {
        username: String::from("no_username"),
        content: String::from("_"),
        reply: false,
        retweet: false,
    });

    if want_tweet { article }
    else { tweet }
}
*/

pub struct Article {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}: [{}]", self.headline, self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: \"{}\"", self.username, self.content)
    }
}
