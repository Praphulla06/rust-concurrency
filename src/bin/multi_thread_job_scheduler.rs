use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let task_queue = Arc::new(Mutex::new(Vec::new()));

    {
        let mut queue = task_queue.lock().unwrap();
        queue.push(10);
        queue.push(20);
        queue.push(30);
    }

    let mut handles = vec![];
    for _ in 0..4 {
        let queue_clone = Arc::clone(&task_queue);
        let handle = thread::spawn(move || {
            loop {
                let task_option = {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.pop()
                };

                match task_option {
                    Some(task) => println!("{}", task),
                    None => break,
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
