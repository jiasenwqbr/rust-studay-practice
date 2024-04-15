use std::mem::size_of;

static B: [u8; 10] = [99, 97, 98, 96, 95, 94, 93, 92, 91, 90];
static C: [u8; 11] = [100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110];
fn main() {
    let a: usize = 42;
    let b = Box::new(&B);
    let c = &C;

    println!("a (unsigned int):");
    println!(" the address is {:p}", &a);
    println!(" the size of a is {:?}", size_of::<usize>());
    println!(" the value is {:?}", a);

    println!("b (unsigned int):");
    println!(" the address is {:p}", &b);
    println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    println!(" the value is {:?}", b);

    println!("c (unsigned int):");
    println!(" the address is {:p}", &c);
    println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    println!(" the value is {:?}", c);

    println!("B (unsigned int):");
    println!(" the address is {:p}", &B);
    println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    println!(" the value is {:?}", B);

    println!("C (unsigned int):");
    println!(" the address is {:p}", &C);
    println!(" the size of a is {:?}", size_of::<Box<[u8]>>());
    println!(" the value is {:?}", C);
}
