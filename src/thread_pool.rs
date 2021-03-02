use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use std::time::Duration;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)-> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing. \n", id);
            if id % 2 == 0 {
                // This seems to be causing the errors on response.send() below
                //thread::sleep(Duration::from_secs(20));
                //println!("Worker {} waited 20 seconds. The other thread should have finished by now! \n", id);
            }else{
                //thread::sleep(Duration::from_secs(10));
                //println!("Worker {} did not wait at all. It should've finished first. \n", id);
            }
            job();
            println!("Worker {} finished executing. \n", id);
        });
        
        Worker{id, thread}
    }
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
    pub fn new(size:usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));            
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F) 
        where F: FnOnce() + Send + 'static, {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
    }
}