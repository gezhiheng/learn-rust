fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 6;

    let y = y + 1; // 变量遮蔽

    {
        let y = y * 2;
        println!("{y}");  // 14
    }

    println!("{y}"); // 7
}
