fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area pf the rectangle is {} quare pixels",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);
    // println!("The area pf the rectangle is {} quare pixels", area(rect1));

    // let retangle = Rectangle {
    //     width: 300,
    //     height: 500,
    // };
    // println!(
    //     "The area pf the rectangle is {} quare pixels",
    //     area(retangle)
    // );
    // println!("retangle is {:?}", retangle);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(demensions: (u32, u32)) -> u32 {
//     demensions.0 * demensions.1
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(retangle: Rectangle) -> u32 {
//     retangle.width * retangle.height
// }
