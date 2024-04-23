//!
//! This crate was created to support Hands on Algorithms and Data Structures With Rust!
//!
//! Chapter 1  
//!
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod door;
fn main() {
    share_state();
}

fn share_state() {
    let v: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers: Arc<Mutex<Vec<i32>>> = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector: std::sync::MutexGuard<'_, Vec<i32>> = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *v.lock().unwrap());
}
