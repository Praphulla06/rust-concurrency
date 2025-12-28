use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

const BUFFER_SIZE: usize = 5;
const ITEM_TO_PRODUCE: usize = 10;

fn main() {
    let buffer = Arc::new((
        Mutex::new(Vec::<usize>::new()),
        Condvar::new(),
        Condvar::new(),
    ));

    let producer_buffer = Arc::clone(&buffer);
    let consumer_buffer = Arc::clone(&buffer);

    let producer = thread::spawn(move || {
        for i in 0..ITEM_TO_PRODUCE {
            let (lock, not_empty, not_full) = &*producer_buffer;

            let mut buf = lock.lock().unwrap();
            while buf.len() == BUFFER_SIZE {
                buf = not_full.wait(buf).unwrap();
            }

            let value = i * 6 * 7;
            println!("{} Produced: {}", i, value);
            buf.push(value);

            not_empty.notify_one();
            drop(buf);
            thread::sleep(Duration::from_millis(200));
        }
    });

    let consumer = thread::spawn(move || {
        for i in 0..ITEM_TO_PRODUCE {
            let (lock, not_empty, not_full) = &*consumer_buffer;

            let mut buf = lock.lock().unwrap();
            while buf.is_empty() {
                buf = not_empty.wait(buf).unwrap();
            }

            let item = buf.remove(0);
            println!("{} Consumed: {}", i, item);

            not_full.notify_one();
            drop(buf);
            thread::sleep(Duration::from_millis(400));
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
}
