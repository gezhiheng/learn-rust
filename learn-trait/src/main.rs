pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// 不能编译因为返回了不同类型
// fn returns_bool_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewsArticle {
//             headline: "headline by henry".to_string(),
//             location: "earth".to_string(),
//             author: "henry ge".to_string(),
//             content: "henry`s article content".to_string(),
//         }
//     }
// }

fn main() {
    let article = NewsArticle {
        headline: "headline by henry".to_string(),
        location: "earth".to_string(),
        author: "henry ge".to_string(),
        content: "henry`s article content".to_string()
    };
    println!("article: {}", article.summarize());
    println!("{}", returns_summarizable().summarize());
}



// fn some_function<T, U>(t: &T, u: &U) 
// where 
//     T: Display + Clone,
//     U: Clone + Debug,
// {

// }