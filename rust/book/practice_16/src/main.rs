fn main() {
    println!("Practice 16.1");
    practice_16_1();
    println!("---------------------------");

    println!("Practice 16.2");
    practice_16_2();
    println!("---------------------------");

    println!("Practice 16.3");
    practice_16_3();
    println!("---------------------------");
}

fn practice_16_1() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("The spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("The main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle_v = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle_v.join().unwrap();
}

fn practice_16_2() {
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx1, rx1) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
    });

    let received = rx1.recv().unwrap();
    println!("Got: {}", received);

    let (tx2, rx2) = mpsc::channel();

    let tx3 = mpsc::Sender::clone(&tx2);
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }
}

fn practice_16_3() {
    use std::sync::{Mutex, Arc};
    use std::thread;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

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
}
