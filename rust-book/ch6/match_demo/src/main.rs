enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coins(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 15,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) {
    match x {
        Some(i) => println!("{}", i),
        _ => (),
    }
}

fn main() {
    println!("Hello, world!");
}
