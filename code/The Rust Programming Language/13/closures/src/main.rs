// use std::{thread, time::Duration};

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }
//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
//     let expensive_clusure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
// }

// fn main() {
//     // let expensive_clusure = |num: u32| -> u32 {
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // };

//     // let num = expensive_clusure(3);
//     // println!("expensive_clusure is {}", num);
//     // fn add_one_v1(x: u32) -> u32 {
//     //     x + 1
//     // }
//     // let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x| x + 1;
//     // let add_one_v4 = |x| x + 1;

//     // let example_closure = |x| x;
//     // let s = example_closure(String::from("hello"));
//     // let n = example_closure(5);

//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//     let only_borrows = || println!("From closure :{:?}", list);
//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure:{:?}", list);
//     let mut borrow_mutably = || list.push(4);
//     borrow_mutably();
//     println!("After calling closure:{:?}", list);
// }

// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     thread::spawn(move || println!("From defining closure :{:?}", list))
//         .join()
//         .unwrap();
//     //println!("after : {:?}", list); //value borrowed here after move
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];
//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("by key called");
//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{:#?}", list);
// }

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    let mut sort_operations = vec![];
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        sort_operations.push(num_sort_operations);
        r.width
    });
    println!(
        "{:#?}, sorted in {num_sort_operations} operations.{:?}",
        list, sort_operations
    );
}
