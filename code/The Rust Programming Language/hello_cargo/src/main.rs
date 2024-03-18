// fn main() {
//     println!("hello world!!! this is my frist rust program");
//     // let r;
//     // {
//     //     let x = 5;
//     //     r = &x; // `x` does not live long enough
//     // }
//     // println!("r is {}", r);
//     // let r;
//     // let x = 5;
//     // r = &x;
//     // println!("r is {}", r);
//     // let string1 = String::from("abcd");
//     // let string2 = "xyz";

//     // let result = longest(string1.as_str(), string2);
//     // println!("The longest string is {}", result);

//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }

//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// #[derive(Debug)]
// struct Vecx {
//     x: f64,
//     y: f64,
// }

// fn main() {
//     let v1 = Vecx { x: 12.0, y: 13.0 };
//     let v2 = Vecx { x: 8.0, ..v1 };
//     let v3 = Vecx { ..v2 };

//     println!("v1 is {:?}", v1);
//     println!("v2 is {:?}", v2);
//     println!("v3 is {:?}", v3);
// }

// struct Number {
//     odd: bool,
//     value: i32,
// }

// fn main() {
//     let one = Number {
//         odd: true,
//         value: 1,
//     };
//     let two = Number {
//         odd: false,
//         value: 2,
//     };
//     print_number2(one);
//     print_number2(two);
// }

// fn print_number(n: Number) {
//     if let Number { odd: true, value } = n {
//         println!("Odd number: {}", value);
//     } else if let Number { odd: false, value } = n {
//         println!("Even number: {}", value);
//     }
// }
// fn print_number2(n: Number) {
//     match n {
//         Number { odd: true, value } => println!("Odd number: {}", value),
//         Number { odd: false, value } => println!("Even number: {}", value),
//     }
// }

// use std::fmt::Debug;

// fn compare<T>(left: T, right: T)
// where
//     T: Debug + PartialEq,
// {
//     println!(
//         "{:?} {} {:?}",
//         left,
//         if left == right { "==" } else { "!=" },
//         right
//     );
// }

// fn main() {
//     compare("tea", "coffee");
//     // prints: "tea" != "coffee"
// }

// struct Pair<T> {
//     a: T,
//     b: T,
// }

// fn print_type_name<T>(_val: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

// fn main() {
//     let p1 = Pair { a: 3, b: 9 };
//     let p2 = Pair { a: true, b: false };
//     print_type_name(&p1);
//     print_type_name(&p2);
// }

// fn main() {
//     let x = 42;
//     let x_ref1 = &x;
//     let x_ref2 = &x;
//     let x_ref3 = &x;
//     println!("{} {} {}", x_ref1, x_ref2, x_ref3);
// }

// fn main() {
//     let mut x = 42;
//     let x_ref = &x;
//     x = 13;
//     println!("x_ref = {}", x_ref);
//     // error: cannot assign to `x` because it is borrowed
// }

// fn main() {
//     let mut x = 42;
//     let x_ref1 = &x;
//     let x_ref2 = &mut x;
//     // error: cannot borrow `x` as mutable because it is also borrowed as immutable
//     println!("x_ref1 = {}", x_ref1);
// }
struct Number {
    value: i32,
}

// fn number_value<'a>(num: &'a Number) -> &'a i32 {
//     &num.value
// }

fn main() {
    let n = Number { value: 47 };
    let v = number_value(&n);
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
    // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.
}
fn number_value(num: &Number) -> &i32 {
    &num.value
}
