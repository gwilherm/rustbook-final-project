// STD
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// Crates
use log::{info, debug};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        info!("Shutting down all workers.");

        for worker in &mut self.workers {
            info!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    debug!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    info!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use std::time::Instant;

    #[test]
    fn mytest() {
        const THREAD_COUNT: usize = 10;
        const TASK_DURATION_SEC: u64 = 2;

        let pool = ThreadPool::new(THREAD_COUNT);
        let start = Instant::now();

        // Keep all workers busy for TASK_DURATION_SEC seconds
        for i in 0..THREAD_COUNT {
            let (tx, rx) = mpsc::channel();

            pool.execute(move || {
                println!("Working...");
                tx.send(i*i).unwrap();
                thread::sleep(Duration::from_secs(TASK_DURATION_SEC));
                println!("Finished!");
            });

            // Check the result
            let result = rx.recv().unwrap();
            println!("{}² = {}", i, result);
            assert_eq!(result, i*i);
        }

        // Give one more task to the pool.
        let (tx, rx) = mpsc::channel();
        pool.execute(move || {
            println!("Waiting to be executed...");
            tx.send(Instant::now()).unwrap();
        });
        let end = rx.recv().unwrap();
        println!("Took {} sec.", (end-start).as_secs());
        assert_eq!((end-start).as_secs(), TASK_DURATION_SEC);
    }
}
