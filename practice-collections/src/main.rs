use std::collections::HashMap;

fn main() {
    let mut nums = vec![2, 3, 1, 5, 6, 8];
    let middle_num = get_middle_num(&mut nums);
    println!("middle_num is :{}", middle_num);

    let nums = vec![2, 3, 2, 3, 6, 6, 6, 10];
    let mode_num = get_mode_num(&nums);
    println!("mode_num is {mode_num}");

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

fn pig_latin(str: &str) {
    
}