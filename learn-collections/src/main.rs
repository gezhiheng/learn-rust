use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];

    let third2 = v[2];

    let forth = v[3];

    println!("{}", third);
    println!("{}", third2);
    println!("{}", forth);

    let s = &"Hello"[0..1];

    println!("{}", s);

    let s2 = "Здравствуйте";

    println!("{}", s2.len());

    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("----------");       

    let mut scores = HashMap::new();

    scores.insert("blue", 10);

    scores.entry("yellow").or_insert(20);

    println!("{scores:?}");
}
