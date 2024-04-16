fn main() {
    // let x = 42;
    // let y = 43;
    // let var1 = &x;
    // let mut var2 = &x;
    // var2 = &y;

    // let mut x;
    // x = 42;
    // let y = &x;
    // println!("{}", &y);
    // let x = 43;
    // //let y = &x;
    // println!("{}", x);
    // println!("{}", y);

    // let x1 = 42;
    // println!("x1 is {:p}", &x1);
    // let y1 = Box::new(84);
    // {
    //     let z = (x1, y1);
    // }
    // let x2 = &x1;
    // let x3 = &x1;
    // println!("x2 is {}", x2);
    // println!("x3 is {}", x3);
    // println!("&x2 is {}", &&x2);
    // println!("&x3 is {}", &&x3);
    // println!("x2 p is {:p}", x2);
    // println!("x3 p is {:p}", x3);
    // println!("&x2 p is {:p}", &x2);
    // println!("&x3 p is {:p}", &x3);
    let mut x = vec![1, 2, 3];
    let mut y = vec![1, 2, 3];
    y.push(5);
    x.append(&mut y);
    println!("{:?}", y);
    println!("{:?}", x);
    let a = String::from_utf8(x).unwrap();
    let b = a;
    //println!("{}", a);
}
