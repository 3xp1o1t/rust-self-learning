use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("rust_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("A new tweet: {}", tweet.summarize());
}
