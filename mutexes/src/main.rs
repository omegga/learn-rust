fn main() {
    create_mutex();
    wont_compile();
    wont_compile_2();
    sharing_mutex_between_threads();
}

fn create_mutex() {
    // Mutex stands for Mutual exclusion and is used to share data across
    // threads, one thread at a time.
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        // To access the data, we use the lock method to acquire the lock.
        // This call will block the current thread until it's our turn to have
        // the lock.
        // The call would fail if another thread holding the lock panicked.
        // Mutex<T> is a smart pointer that returns a pointer called MutexGuard,
        // wrapped in a LockResult (what is returned by unwrap).

        // MutexGuard implements Deref (allows to point at inner data)
        // and Drop (releases the lock automatically at the end of the scope).
        // This prevents us from forgetting to release the lock and blocking
        // the mutex from being used.

        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn wont_compile() {
    // We can't move the ownership of counter into multiple threads

    // use std::sync::Mutex;
    // use std::thread;
    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
}

fn wont_compile_2() {
    // Rc<T> is not safe to share across threads

    // use std::rc::Rc;
    // use std::sync::Mutex;
    // use std::thread;
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
}

fn sharing_mutex_between_threads() {
    // Because Rc<T> is not safe to use between threads, we have to use Arc<T>
    // (atomically reference counted)
    use std::sync::{Arc, Mutex};
    use std::thread;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mutex<T> provides interior mutability, allowing us to mutate the
            // contents inside it, just like RefCell<T> inside an Rc<T>.
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result = {}", *counter.lock().unwrap());

    // Just like Rc<T> came with the risk of creating reference cycles, where
    // two Rc<T> values refer to each other, Mutex<T> comes with the risk of
    // creating deadlocks.
    // These occur when an operation needs to lock two resources and two threads
    // have each acquired one of the locks, causing them to wait for each other
    // forever.
}
