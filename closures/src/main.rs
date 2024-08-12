use::std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
 width: u32,
 height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || { 
        list.push(7);
    };

    // println!("Closure: {list:?}"); 闭包的定义和调用之间不能有闭包中所可变引用变量的引用

    thread::spawn(move || println!("From thread: {list:?}")) // 所有权转移
        .join()
        .unwrap();

    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    let mut list = [
        Rectangle{ width: 10, height: 1 },
        Rectangle{ width: 3, height: 5 },
        Rectangle{ width: 7, height: 12 },
    ];

    // let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|k| {
        // sort_operations.push(value); 
        k.width
    });

    println!("list after sort: {list:#?}");
}
