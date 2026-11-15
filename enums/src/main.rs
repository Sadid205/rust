// fn main() {
//     println!("Hello, world!");
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

// fn main() {
//     let four = IpAddrKind::V4(127, 0, 0, 1);
//     println!("IP Address: {:?}", four);
// }
 


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Alibaba,   // আপনি যেহেতু Alibaba ব্যবহার করছেন, সেটাও যোগ করুন
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
            println!("State quarter from {:?}!", state);
            25
        }
    }
}



fn main () {
    println!("Value of a Dime: {}", value_in_cents(Coin::Quarter(UsState::Alibaba)));
}