use std::{thread, time::Duration};

fn main() {
    let thread_01 = thread::spawn(|| {
        for _ in 0..10 {
            println!("Hello, from thread 01");
            thread::sleep(Duration::from_millis(300));
        }
    });

    for _ in 0..10 {
        println!("Hello, from main thread!");
        thread::sleep(Duration::from_millis(100));
    }

    thread_01.join().unwrap();

    // move
    let name = String::from("Kenny");
    let thread_02 = thread::spawn(move || {
        println!("User's name: {}", name);
        thread::sleep(Duration::from_millis(200));
    });
    thread_02.join().unwrap();

    // multiple threads
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Hello, from spawned thread {}", i);
            thread::sleep(Duration::from_millis(400));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
