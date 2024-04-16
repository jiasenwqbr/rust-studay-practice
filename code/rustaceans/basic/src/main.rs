use std::ops::Deref;

fn main() {
    let x = 5;
    let y = Box::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = 5;
    let b = MyBox::new(5);
    assert_eq!(5, a);
    assert_eq!(5, *(b.deref()));
    assert_eq!(5, *b);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
