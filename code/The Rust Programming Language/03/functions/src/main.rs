fn main() {
    // another_function(5);
    // let x = (let y=6);

    /* let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); */
    /* let x = five();
    println!("The value of x is: {x}"); */
    let x = plus_one(5);
    println!("the value of x is :{x}")
}
// fn another_function(x: i32) {
//     println!("the value of x is :{x}")
// }

/* fn five() -> i32 {
    5
} */

fn plus_one(x: i32) -> i32 {
    x + 1
}
