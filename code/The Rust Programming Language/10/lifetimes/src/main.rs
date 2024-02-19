// fn main() {
// let r;
// {
//     let x = 5;
//     r = &x;
// }
// println!("r is {}", r);

// let string1: String = String::from("abcd");
// let string2 = "xyz";

// let result = longest(string1.as_str(), string2);
// println!("The longest string is {}", result);

// let string1 = String::from("long string is long");
// {
//     let string2 = String::from("xyz");
//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is {}", result);
// }

//error[E0597]: `string2` does not live long enough
// let string1 = String::from("long string is long");
// let result;
// {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
// }
// println!("The longest string is {}", result);
//}
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
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

struct ImportantExcept<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i: ImportantExcept<'_> = ImportantExcept {
        part: first_sentence,
    };
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
