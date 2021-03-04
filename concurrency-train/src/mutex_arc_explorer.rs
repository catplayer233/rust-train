use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_api() {
    let mutex = Mutex::new(6);
    {
        let mut mutex_guard = mutex.lock().unwrap();
        *mutex_guard = 8;
    }

    println!("mutex = {:?}", mutex);
}

pub fn multi_thread_mutex() {
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let mutex_arc = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut num = mutex_arc.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *mutex.lock().unwrap());
}