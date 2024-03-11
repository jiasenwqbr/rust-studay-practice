fn main() {
    println!("hello world!!! this is my frist rust program");
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // `x` does not live long enough
    // }
    // println!("r is {}", r);
    // let r;
    // let x = 5;
    // r = &x;
    // println!("r is {}", r);
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
