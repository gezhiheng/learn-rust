fn main() {
    let s2tr = String::from("hello world");
    println!("{}", first_world(&s2tr));
    println!("{}", s2tr);

    let s = "hello world";

    let s2 = &s[..6];

    println!("{}", s2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}
