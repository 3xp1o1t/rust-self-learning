use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("rust_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("A new tweet: {}", tweet.summarize());

    // default implementation of a trait
    let article = NewsArticle {
        headline: String::from("Los pinguinos aman linux"),
        location: String::from("kernel de linux"),
        author: String::from("JMontiel"),
        content: String::from("En el polo norte existe un pinguino llamado Linus que esta fumado."),
    };

    println!("New article available! {}", article.summarize());
}
