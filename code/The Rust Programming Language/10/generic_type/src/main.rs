//use aggregator::{Summary, Tweet};
fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let largest_num = largest(&number_list);
    // println!("The largest number is {}", largest_num);

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // let p = Point { x: 5, y: 10 };

    // println!("p.x = {}", p.x());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     largest
// }
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
