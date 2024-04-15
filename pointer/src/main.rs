use std::{
    borrow::Cow,
    ffi::{c_char, CStr},
    mem::size_of,
};

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    // let a: usize = 42;
    // let b = Box::new(&B);
    // let c = &C;

    // println!("a (unsigned int):");
    // println!(" the address is {:p}", &a);
    // println!(" the size of a is {:?}", size_of::<usize>());
    // println!(" the value is {:?}", a);

    // println!("b (unsigned int):");
    // println!(" the address is {:p}", &b);
    // println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    // println!(" the value is {:?}", b);

    // println!("c (unsigned int):");
    // println!(" the address is {:p}", &c);
    // println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    // println!(" the value is {:?}", c);

    // println!("B (unsigned int):");
    // println!(" the address is {:p}", &B);
    // println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    // println!(" the value is {:?}", B);

    // println!("C (unsigned int):");
    // println!(" the address is {:p}", &C);
    // println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    // println!(" the value is {:?}", C);

    let a = 42;
    let b: String;
    let c: Cow<str>;
    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {},b:{},c:{}", a, b, c);
}
