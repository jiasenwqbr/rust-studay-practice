// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//     println!("m = {:?}", m);
// }

// use std::{sync::Mutex, thread};

// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         // value moved into closure here, in previous iteration of loop
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result:{}", *counter.lock().unwrap());
// }

// use std::{rc::Rc, sync::Mutex, thread};

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num: std::sync::MutexGuard<'_, i32> = counter.lock().unwrap();
//             *num += 1;
//         });
//         // we compile and get... different errors! The compiler is teaching us a lot
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<_> = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
