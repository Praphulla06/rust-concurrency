use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let value = Arc::new(Mutex::new(10));

    let thread_01_value = Arc::clone(&value);
    let thread_01 = thread::spawn(move || {
        for i in 0..10 {
            let mut value = thread_01_value.lock().unwrap();
            *value += i;
            println!("Updated value from thread 01");
            drop(value);
            thread::sleep(Duration::from_millis(100));
        }
    });
    let thread_02_value = Arc::clone(&value);
    let thread_02 = thread::spawn(move || {
        for i in 0..10 {
            let mut value = thread_02_value.lock().unwrap();
            *value += i * 101;
            println!("Updated value from thread 02");
            drop(value);
            thread::sleep(Duration::from_millis(150));
        }
    });

    thread_01.join().unwrap();
    thread_02.join().unwrap();
    println!("{:#?}", value);
}
