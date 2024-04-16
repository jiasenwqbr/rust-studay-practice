use rand::prelude::*;
fn main() {
    let mut x = Box::new(42);
    let r = &x;
    if random::<f32>() > 0.5 {
        *x = 84;
        // println!("{}", r);
    } else {
        println!("{}", r);
    }

    // let a = 123_2;
    // let a_ = 123_8.223_223;
    // println!("{}", a);
    // println!("{}", a_);

    // let tup: (u32, i32, f32, f64, char) = (1, 2, 3.0, 4.0, '字');
    // let (a, b, c, d, e) = tup;
    // println!("{},{},{},{},{}", a, b, c, d, e);

    // let s = "fdashfhahjkf";
    // let s1 = String::from("hello");
    // // let sa = &mut s1;
    // let s2 = s1;
    // let mut s3 = String::from("hello");
    // let s4 = &mut s3;
    // {
    //     let s5 = &mut s3;
    // }

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let mut s3 = s1 + &s2;
    println!("{}", s3);
    let s0 = String::from(" hahah ");
    s3.push_str(&s0);
    s3.push('q');
    println!("{}", s3);
}

// fn returnstr() -> String {
//     let s = String::from("hello");
//     s
// }
// fn return_str() -> &'static str {
//     let s = String::from("hello");
//     "hello"
// }
