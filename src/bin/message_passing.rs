use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    // single value message passing
    let (tx_01, rx_01) = mpsc::channel();

    // let tx = tx_01.clone();
    let thread_01 = thread::spawn(move || {
        let value = String::from("Lysa");
        tx_01.send(value).unwrap();
    });

    let thread_02 = thread::spawn(move || {
        let recieved = rx_01.recv().unwrap();
        println!("Name: {}", recieved);
    });

    thread_01.join().unwrap();
    thread_02.join().unwrap();

    // multiple sender single reciever
    let (tx_02, rx_02) = mpsc::channel();

    let tx_021 = tx_02.clone();
    let thread_03 = thread::spawn(move || {
        let fruits = vec!["Apple", "Banana", "Avocado", "Cherry"];
        for fruit in fruits {
            tx_021.send(fruit).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    let thread_04 = thread::spawn(move || {
        let vegetables = vec!["Potato", "Brocolli", "Beetroot", "Spanich"];
        for veg in vegetables {
            tx_02.send(veg).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    let thread_05 = thread::spawn(move || {
        for eatable in rx_02 {
            println!("Got: {}", eatable);
        }
    });

    thread_03.join().unwrap();
    thread_04.join().unwrap();
    thread_05.join().unwrap();
}
