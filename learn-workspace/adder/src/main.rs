use add_one;
use std::collections::HashMap;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    let mut map = HashMap::<i32, i32>::new();

    map.insert(1, 1);

    let v = map.entry(1).or_insert(0);
    println!("v: {v:?}");

    println!("map: {map:?}");
}
