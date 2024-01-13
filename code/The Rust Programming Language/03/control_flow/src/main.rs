fn main() {
    // let number = 8;
    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition is false")
    // }

    // error: could not compile `branches` due to previous error
    // let number = 3;
    // if number {
    //     println!("number was three")
    // }

    // let number = 6;
    // if number % 4 == 0 {
    //     print!("number is divisable by 4 ");
    // } else if number % 3 == 0 {
    //     println!("number is divisable by 3")
    // } else if number % 2 == 0 {
    //     println!("number is divisable by 2")
    // } else {
    //     println!("number is not divisable by 4,3,2")
    // }

    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("the value of the number is : {number}");

    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");
    // loop {
    //     // let number = 6;
    //     // println!("The value of number is: {number}");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("the result is {result}")

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("{number}");
    // println!("LIFTOFF!")

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("{element}")
    // }

    for number in (1..4).rev() {
        println!("{number}")
    }
    println!("LIFTOFF!")
}
