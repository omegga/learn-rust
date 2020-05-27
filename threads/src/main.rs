fn main() {
    println!("Hello, world!");
    creating_a_thread();
    waiting_for_threads_to_finish();
    waiting_for_threads_to_finish_2();
    wont_compile();
    move_values();
    sending_messages();
    sending_messages_2();
    sending_messages_3();
}

fn creating_a_thread() {
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn waiting_for_threads_to_finish() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn waiting_for_threads_to_finish_2() {
    use std::thread;
    use std::time::Duration;
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn wont_compile() {
    // use std::thread;
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {
    //     println!("v = {:?}", v);
    // });
}

fn move_values() {
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("v = {:?}", v);
    });

    // drop(v); // this won't work, as v was moved into the closure

    handle.join().unwrap();
}

fn sending_messages() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
        // this won't work because val was moved in the main thread
        // println!("val = {}", val);
    });

    // This will block the main thread execution and wait until a value is sent
    // down the channel.
    // recv (for "receive") will return it in a Result<T, E>
    let received = rx.recv().unwrap();
    println!("got {}", received);

    // When the sendind end closes, recv will return an error to signal that
    // no more values will be coming.

    // the try_recv method doesn't block but will instead return Result<T, E>
    // immediately: an Ok value holding a message if one is available, and an
    // Err value if there aren't any messages this time.
    // Using try_recv is useful if this thread has other work to do.
    // We could use try_recv in a loop and call it every time we need to
    // retrieve a message while doing other work.
}

fn sending_messages_2() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {}", received);
    }
}

fn sending_messages_3() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    println!("");

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
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
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got {}", received);
    }
}
