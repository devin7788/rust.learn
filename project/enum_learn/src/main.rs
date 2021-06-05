#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    println!("Hello, world!");
    println!("IpAddrKind::V4 = {:#?}", IpAddrKind::V4);
    println!("IpAddrKind::V6 = {:#?}", IpAddrKind::V6);


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home = {:#?}", home);
    println!("coin={}", value_in_cents(Coin::Penny));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
