use std::thread;
use std::time::Duration;

mod mutex_arc_explorer;

fn main() {
    //the api:spawn will execute a closure when current thread execute stopped but not ended.
    //if current thread ended, the spawn thread will ended
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //join can make current thread's execution stopped util the join_handle's thread ended
    join_handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::sync::mpsc::Sender;
    use std::thread;
    use std::time::Duration;

    use crate::mutex_arc_explorer::{multi_thread_mutex, mutex_api};

    #[test]
    fn move_closure_test() {
        let vec = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", vec);
        });

        handle.join().unwrap();
    }

    #[test]
    fn message_passing_test() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            //the val's ownership was moved by tx.send, you can not use val again
            // println!("val is {}", val);
        });

        let val = rx.recv().unwrap();
        println!("Got: {}", val);
    }

    #[test]
    fn multi_value_send_test() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let contents = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for content in contents {
                tx.send(content).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        //the receiver has Iterator trait do the iter function will receive every message when it received
        for received in rx {
            println!("Got {}", received);
        }
    }

    #[test]
    fn multi_producer_by_clone_test() {
        let (tx, rx) = mpsc::channel();
        let tx_copy = tx.clone();
        thread::spawn(move || {
            do_send(tx_copy);
        });

        thread::spawn(move || {
            do_send(tx);
        });

        for received in rx {
            println!("Got {}", received);
        }
    }

    fn do_send(tx: Sender<String>) {
        let contents = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for content in contents {
            tx.send(content).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }

    #[test]
    fn mutex_test() {
        mutex_api();
        multi_thread_mutex();
    }
}
