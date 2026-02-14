use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    println!("Hello from treads.rs");
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            for i in 0..200 {
                *count += 1;
            }
        });

        handles.push(handle);
    }

    for i in handles {
        i.join().unwrap();
    }
    
    println!("Counter Value is: {}",*mutex.lock().unwrap());
}