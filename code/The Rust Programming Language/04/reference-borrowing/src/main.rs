//use std::mem;

fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}", s1, len);

    // // 定义两个字符串变量
    // let mut str1 = String::from("Hello");
    // let mut str2 = String::from("World");

    // // 打印交换前的字符串
    // println!("Before Swap: str1 = {}, str2 = {}", str1, str2);

    // // 使用 drain 方法交换字符串内容
    // let temp: String = str1.drain(..).collect();
    // str1 = str2.drain(..).collect();
    // str2 = temp;

    // // 打印交换后的字符串
    // println!("After Swap: str1 = {}, str2 = {}", str1, str2);

    // let mut str1 = String::from("Hello");
    // let mut str2 = String::from("World");

    // println!("Before Swap: str1 = {}, str2 = {}", str1, str2);

    // // 使用 std::mem::swap 交换两个字符串
    // mem::swap(&mut str1, &mut str2);

    // println!("After Swap: str1 = {}, str2 = {}", str1, str2);

    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("the s is {}", s);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    //              //let r3 = &mut s; // BIG PROBLEM
    // let r3 = &s;

    // println!("{}, {}, and {}", r1, r2, r3);
    let reference_to_nothing = dangle();
    println!("reference_to_nothing is {}", reference_to_nothing);
}
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(",world");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");

    s
}
