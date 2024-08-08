use std::fmt::Display;

fn main() {
    let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     // result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
    let string2 = "xyz";
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone`s birthday!",
    );
    println!("The longest string is {result}");
}

// `a 表示生命周期 两个参数和返回值引用 都是同一个生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}