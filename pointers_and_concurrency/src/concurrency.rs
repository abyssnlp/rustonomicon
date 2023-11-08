use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // Thread spawn closure
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Vector: {v:?}");
    });

    handle2.join().unwrap();

    // Message passing

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("More"),
            String::from("and more"),
            String::from("and yet more"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("ultraman"),
            String::from("shinjiro"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }

    // Shread thread concurrency: Mutex and Arc<T>
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

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

    // Extensible Concurrency with Send and Sync
    // Send: Can be transferred between threads
    // Sync: atomic
}
