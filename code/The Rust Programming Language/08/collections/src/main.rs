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

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // {
    //     let v = vec![1, 2, 3, 4];

    //     // do stuff with v
    // } // <- v goes out of scope and is freed here

    ///////////////////////////////////////////////////////
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // println!("The result is :{}", s);

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // let mut s = String::from("Hello");
    // s.push_str(" world!");
    // println!("{}", s);
    // let s3 = "my name is jason";
    // s.push_str(s3);
    // println!("{}", s)
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // //let s = s1 + "-" + &s2 + "-" + &s3;
    // //let s = format!("{s1}_{s2}_{s3}");
    // let s = format!("{}_{}_{}", s1, s2, s3);
    // println!("{s}")

    let hello = "hello";
    // let answer = &hello[1..10];
    // println!("{}", answer);
    for a in hello.chars() {
        println!("{a}")
    }
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
