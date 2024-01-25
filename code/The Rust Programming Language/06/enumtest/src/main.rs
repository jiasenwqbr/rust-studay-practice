// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

/////////////////////////////////////////////////////////////////
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {}

/////////////////////////////////////////////////////////////////
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             print!("Penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn main() {}

/////////////////////////////////////////////////////////////////
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     let num = value_in_cents(coin);
//     println!("value in cents is {}", num);
// }

/////////////////////////////////////////////////////////////////
///
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("five is {:?} ,none is {:?} ", six, none);
// }

/////////////////////////////////////////////////////////////////

// fn add_fancy_hat() {}
// fn remove_fancy_hat() {}
// fn reroll() {}

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }
// }

/////////////////////////////////////////////////////////////////
fn main() {
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }
}
