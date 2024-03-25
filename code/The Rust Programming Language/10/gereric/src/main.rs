// fn func<T>(arg: T) -> T {
//     arg
// }

// fn main() {
//     let x = func(30);
//     println!("x is {:?}", x);
//     let x = func("i am jason");
//     println!("x is {:?}", x);
//     let x = func(String::from("I love rust"));
//     println!("x is {:?}", x);
//     let x = func(vec![1, 2, 3, 4]);
//     println!("x is {:?}", x);
// }

// fn func<A, B, C>(arg1: A, arg2: B, arg3: C) -> (A, C) {
//     (arg1, arg3)
// }
// fn main() {
//     let x = func("Jason", 256, " Hello");
//     println!("{:?}", x);
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let p1 = Point { x: 11, y: 22 };
//     let p2 = Point { x: 11.1, y: 22.2 };

//     println!("p1 x is {:?} y is {:?}", p1.x, p1.y);
//     println!("p2 x is {:?} y is {:?}", p2.x, p2.y);
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// fn main() {
//     let p1 = Point { x: 11, y: "十三" };
//     let p2 = Point {
//         x: 13,
//         y: String::from("三十三"),
//     };
//     println!("p1 x is {:?} y is {:?}", p1.x, p1.y);
//     println!("p2 x is {:?} y is {:?}", p2.x, p2.y);
//     let p3 = Point {
//         x: vec![1, 2, 3],
//         y: (1, 2, 3),
//     };
//     println!("p3 x is {:?} y is {:?}", p3.x, p3.y);
// }

// enum MyOption<A, B, C> {
//     Some1(A),
//     Some2(B),
//     Some3(C),
// }
// fn main() {
//     let x: MyOption<i32, f64, u8> = MyOption::Some1(123);
//     match x {
//         MyOption::Some1(value) => println!("我是i32：{}", value),
//         MyOption::Some2(value) => println!("我是f64：{}", value),
//         MyOption::Some3(value) => println!("我是u8：{}", value),
//     }
//     let y: MyOption<u8, i32, String> = MyOption::Some3(String::from("xxx"));
//     match y {
//         MyOption::Some1(v) => println!("我是 u8:{}", v),
//         MyOption::Some2(v) => println!("我是 i32 : {}", v),
//         MyOption::Some3(v) => println!("我是 String : {}", v),
//     }
//     let z: MyOption<u8, i32, String> = MyOption::Some2(34);
//     match z {
//         MyOption::Some1(v) => println!("我是 u8 {}", v),
//         MyOption::Some2(v) => println!("我是 i32 {}", v),
//         MyOption::Some3(v) => println!("我是 String {}", v),
//     }
// }

// enum MyOption<T> {
//     MySome1(T),
//     MySome2(i32),
//     MySome3(T),
//     MyNone,
// }

// fn main() {
//     // 这里我们没有指定 x 的类型
//     // 这是因为 MyOption 只有一个泛型
//     // 通过给 MySome1 传递的值，可以推断出 T 的类型
//     let x = MyOption::MySome1(123);

//     // 同样的道理，Rust 可以自动推断，得出 T 是 &str
//     let x = MyOption::MySome3("123");

//     // 但此处就无法自动推断了，因为赋值的是 MySome2 成员
//     // 此时 Rust 获取不到任何有关 T 的信息，无法执行推断
//     // 因此我们需要手动指定类型，但仔细观察一下声明
//     // 首先，如果没有泛型的话，那么直接 let x: MyOption = ... 即可
//     // 但里面有泛型，所以此时除了类型之外，还要连同泛型一起指定
//     // 也就是 MyOption<f64>
//     let x: MyOption<f64> = MyOption::MySome2(123);

//     // 当然泛型可以代表任意类型，此时的 T 则是一个 Vec<i32> 类型
//     let x: MyOption<Vec<i32>> = MyOption::MySome2(123);
// }

// struct Girl1 {
//     field: i32,
// }

// struct Girl2<T> {
//     field: T,
// }

// fn main() {
//     // 下面两个语句类似，只是第二次声明 g1 的时候多指定了类型
//     let g1 = Girl1 { field: 123 };
//     let g1: Girl1 = Girl1 { field: 123 };

//     // 下面两条语句也是类似的，第二次声明 g2 的时候多指定了类型
//     // 但此时的类型有些不一样，Girl2 的结尾多了一个 <i32>
//     // 原因很简单，因为 Girl2 里面有泛型
//     // 所以在显式指定类型的时候，还要将泛型代表的类型一块指定，否则报错
//     let g2 = Girl2 { field: 123 };
//     let g2: Girl2<i32> = Girl2 { field: 123 };
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<K> Point<K, f64> {
//     fn m1(&self) {
//         println!("I am the m1 method");
//     }
// }
// impl<T, U> Point<T, U> {
//     fn m2(&self) {
//         println!("我是 m2 方法")
//     }
// }
// impl Point<u8, f64> {
//     fn m3(&self) {
//         println!("我是 m3 方法")
//     }
// }
// fn main() {
//     // 显然 p1 可以同时调用 m1 和 m2 方法，但 m3 不行
//     // 因为 m3 要求 T 是一个 u8，而当前是 &str
//     let p1 = Point {
//         x: "hello",
//         y: 3.14,
//     };
//     p1.m1(); // 我是 m1 方法
//     p1.m2(); // 我是 m2 方法

//     // 显然 p2 可以同时调用 m1、m2、m3
//     // 另外这里的 x 可以直接写成 123，无需在结尾加上 u8
//     // 因为 Rust 看到我们调用了 m3 方法，会自动推断为 u8
//     let p2 = Point { x: 123u8, y: 3.14 };
//     p2.m1(); // 我是 m1 方法
//     p2.m2(); // 我是 m2 方法
//     p2.m3(); // 我是 m3 方法

//     // 显然 p3 只能调用 m2 方法，因为 m2 对 T 和 W 没有要求
//     // 但是像 m3 就不能调用，因为它是为 <u8, f64> 实现的方法
//     // 只有当 T、W 为 u8、f64 时才可以调用
//     // 显然此时是不满足的，因为都是 &str，至于 m1 方法也是同理
//     // 所以 p3 只能调用 m2，这个方法是为 <K, S> 实现的
//     // 而 K 和 S 也是泛型，可以代表任意类型，因此没问题
//     let p3: Point<&str, &str> = Point {
//         x: "3.14",
//         y: "123",
//     };
//     p3.m2(); // 我是 m2 方法
// }
enum MyOption<T> {
    MySome1(T),
    MySome2(i32),
    MySome3(T),
    MyNone,
}

fn main() {
    let x: MyOption<i32> = MyOption::MySome1(123);
    if let MyOption::MySome1(value) = x {
        println!("the x value is {}", value);
    }
}
