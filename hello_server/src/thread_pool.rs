use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || loop {
                let msg = receiver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("Worker {id}: received a job.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id}: is closed.");
                        break;
                    }
                }
            })),
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    /// # Panics
    /// The 'new' function will panic if passed 0
    pub fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(thread_count);
        for i in 0..thread_count {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Dropping worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            };
        }
    }
}
