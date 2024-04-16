fn main() {
    let mut x;
    x = 42;
    let y = &x;
    x = 43;
    println!("{}", *y);
}
