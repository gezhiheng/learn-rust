#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {state:?} !");
            25
        },
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let dice_roll = 9;

    match dice_roll {
        1 => {
            println!("one");
        },
        other => {
            println!("{}", other);
        }
    }

    let some_number = Some(5);
    println!("{some_number:?}");

    // refactor by if let 
    if let 1 = dice_roll {
        println!("if let one");
    } else {
        println!("if let other");
    }

}
