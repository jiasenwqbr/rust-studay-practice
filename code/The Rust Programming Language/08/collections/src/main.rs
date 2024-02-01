fn main() {
    // // creating a new ,empty vector to hold values of type i32
    // let mut v: Vec<i32> = Vec::new();
    // let v2 = vec![1, 2, 3];
    // v.push(5);
    // v.push(3);
    // v.push(4);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // println!("The value is {does_not_exist}");
    // let does_not_exist = v.get(100);
    // match does_not_exist {
    //     Some(does_not_exist) => println!("The value is {does_not_exist}"),
    //     None => print!("not exist"),
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in v {
    //     println!("The first element is: {i}");
    // }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
