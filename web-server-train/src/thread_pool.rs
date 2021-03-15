use std::sync::{Arc, mpsc, Mutex};
use std::thread::JoinHandle;
use std::thread::spawn;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<JobMessage>,

}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum JobMessage {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> Self {
        assert!(pool_size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(pool_size);

        for id in 0..pool_size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(JobMessage::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(JobMessage::Terminate).unwrap();
        }
        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<JobMessage>>>) -> Self {
        let thread = spawn(move || loop {
            let job_message = receiver.lock().unwrap().recv().unwrap();

            match job_message {
                JobMessage::NewJob(job) => {
                    println!("Worker {} get a job; executing.", id);
                    job();
                }
                JobMessage::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}