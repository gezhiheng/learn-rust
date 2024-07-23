fn main() {
    let s1 = String::from("hello");

    let (s2, len) = caculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    // println!("{}", s1);

    let str1 = String::from("hello");

    takes_ownership(&str1);

    println!("{}", str1);

    let mut name = String::from("henry");

    chang_name(&mut name);

    chang_name(&mut name);

    let name_r = &mut name;

    println!("{}", name_r);

    // println!("{}", &mut name);

    let name_r2 = &mut name;

    println!("{name_r2}");
    
    let string11 = no_dangle();

    println!("{string11}");

}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn chang_name(name: &mut String) {
    name.push_str("awesome");
}

fn takes_ownership(some_str: &String) {
    println!("{}", some_str);
}

fn caculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
