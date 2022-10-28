use chapter10_2::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebokks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        content: String::from("great content"),
        headline: String::from("headline"),
        location: String::from("Japan"),
        author: String::from("Bob"),
    };

    println!("1 new article: {}", article.summarize());
}
