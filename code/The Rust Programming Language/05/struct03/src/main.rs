// #[derive(Debug)]
// struct Retangle {
//     width: u32,
//     height: u32,
// }
// impl Retangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let rect1 = Retangle {
//         width: 300,
//         height: 500,
//     };
//     println!(
//         "The area of the retangle is {} square pixels.",
//         rect1.area()
//     );
// }

////////////////////////////////////////////////////////////////////////////
// #[derive(Debug)]
// struct Retangle {
//     width: u32,
//     height: u32,
// }
// impl Retangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
// fn main() {
//     let rect1 = Retangle {
//         width: 30,
//         height: 50,
//     };
//     if (rect1.width()) {
//         println!("The rectangle has a nonzero width ;it is {}", rect1.width)
//     }
// }

////////////////////////////////////////////////////////////////////////////

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2: Rectangle = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };
//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!(
//         "The area of  rect1 is  {},The area of rect2 is {},The area of rect3 is {}",
//         rect1.area(),
//         rect2.area(),
//         rect3.area()
//     );
// }
////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of  rect1 is  {},The area of rect2 is {},The area of rect3 is {}",
        rect1.area(),
        rect2.area(),
        rect3.area()
    );
    let area = Rectangle::square(8);
    println!("height is  :{},width is: {}", area.height, area.width);
}
