use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;
fn main() {
    // panic!("crash and burn");
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
    // println!("{:?}", greeting_file);

    // let greeting_file_result = File::open("hello.txt");
    // let greeeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    // let a = read_username_from_file();
    // println!("{:?}", a);
    // let b = read_username_from_file_1();
    // println!("{:?}", b);

    // let greeting_file: File = File::open("hello.txt")?;
    // println!("{:?}", greeting_file);

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{:?}", home);
    let space_str = "11wretw2";
    let space_str = space_str.len();
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file_1() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }

// }
