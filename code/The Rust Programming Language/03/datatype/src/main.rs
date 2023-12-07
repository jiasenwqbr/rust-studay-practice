use std::io;
fn main() {
    /* let guess: u32 = "42".parse().expect("Not a number!");
    println!("Hello, world! {guess}"); */
    /* let x = 2.0;
    let y: f32 = 3.0;
    println!("{x}");
    println!("{y}");

    let z: f32 = x / y;
    println!("{z}"); */

    /* // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("sum is : {sum}");
    println!("difference is {difference}");
    println!("product is {product}");
    println!("quotient is {quotient}");
    println!("truncated is {truncated}");
    println!("remainder is : {remainder}"); */

    /* let t = true;
    let f: bool = false;
    println!("t is {}", t);
    println!("f is {}", f); */

    /* let c = 'z';
    let z: char = 'ℤ';
    let heart_eye_cat = '😻';
    println!("c is {}", c);
    println!("z is {}", z);
    println!("heart_eye_cat is {}", heart_eye_cat); */

    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("x is {} y is {} z is {}", x, y, z);

    /* let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "five_hundred is {} six_point_four is {} one is {}",
        five_hundred, six_point_four, one
    ); */
    /*
    let a = [1, 2, 3, 4, 5];
    println!("a is :{}", a[0]); */

    let a = [1, 2, 3, 4, 5];
    println!("please input a number");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
