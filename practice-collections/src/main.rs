use std::collections::HashMap;

fn main() {
    let mut nums = vec![2, 3, 1, 5, 6, 8];
    let middle_num = get_middle_num(&mut nums);
    println!("middle_num is :{}", middle_num);

    let nums = vec![2, 3, 2, 3, 6, 6, 6, 10];
    let mode_num = get_mode_num(&nums);
    println!("mode_num is {mode_num}");

    let word = pig_latin(&"apple");
    println!("pig latin word is {word}");

    employee(&vec!["Add Sally to Engineering".to_string(), "Add Amir to Sales".to_string(), "Add Henry to Engineering".to_string()])
}

fn get_middle_num(vec: &mut Vec<i32>) -> i32 {
    vec.sort();
    let index = vec.len() / 2;
    vec[index]
}

fn get_mode_num(vec: &Vec<i32>) -> &i32 {
    let mut count_map = HashMap::new();

    for i in vec {
        let count = count_map.entry(i).or_insert(0);
        // TODO WTF??
        *count += 1;
    }

    let mut res_key: &i32 = &0;

    let mut res_val: &i32 = &0;

    for (key, val) in &count_map {
        if val > res_val {
            res_val = val;
            res_key = key;
        }
    }

    res_key
}

fn pig_latin(str: &str) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&str.chars().next().unwrap()) {
        format!("{}-hay", str)
    } else {
        let mut words = str.chars();
        let first_word = words.next().unwrap();
        let rest_words: String = words.collect();

        format!("{}-{}ay", first_word, rest_words)
    }
}

fn employee(str_vec: &Vec<String>) {
    let mut empolyee_map = HashMap::<String, Vec<String>>::new();
    for input in str_vec {
        let v: Vec<&str> = input.split(' ').collect();
        let vec = Vec::<String>::new();
        empolyee_map.entry(v[3].to_string()).or_insert(vec).push(v[1].to_string());
    }
    println!("empolyee_map is {empolyee_map:?}");
}
