use core::slice;

fn main() {
    // let mut s: String = String::from("hello world");
    // let index = first_word(&s);
    // s.clear();
    // println!("The index of blank is {}", index);
    // println!("ss {}", s);

    // let s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..10];
    // println!("s is  {} ,hello is {},world is {}", s, hello, world);

    // let s = String::from("hello world");
    // let len = s.len();
    // let sl2 = &s[0..len];
    // let sl2 = &s[..];
    // println!("s is  {} ,hello is {}", s, sl2);

    // let s = String::from("hello world");
    // let frist = frist_world(&s);
    // println!("first  is  {} ", frist);

    let my_string = String::from("hello world");
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
//}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
