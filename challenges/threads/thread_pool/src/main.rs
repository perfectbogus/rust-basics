use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

type WorkItem = Box<dyn FnOnce() -> i32 + Send>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkItem>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<WorkItem>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender}
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() -> i32 + Send + 'static
    {
        let job = Box::new(job);

        self.sender.send(job).unwrap();
    }

    fn execute_batch(&self, jobs: Vec<WorkItem>) -> Vec<i32> {
        use std::sync::mpsc;

        let (result_sender, result_receiver) = mpsc::channel::<(usize, i32)>();

        for (index, job) in jobs.into_iter().enumerate() {
            let sender_clone = result_sender.clone();

            let wrapped_job = Box::new(move || {
                let result = job();
                sender_clone.send((index, result)).unwrap();
                result
            });

            self.sender.send(wrapped_job).unwrap();
        }

        drop(result_sender);

        let mut results = Vec::new();
        while let Ok((index, result)) = result_receiver.recv() {
            results.push((index, result));
        }

        results.sort_by_key(|&(index, _)| index);
        results.into_iter().map(|(_, result)| result).collect()
    }

    fn shutdown(self) {
        // Drop the sender to signal the workers to shut down
        drop(self.sender);

        //Join the worker threads
        for worker in self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Err(e) = worker.thread.join() {
                eprintln!("Worker {} panicked: {:?}", worker.id, e);
            }
        }

        println!("All workers shut down successfully");
    }

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkItem>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => {
                        println!("Worker {} execution job", id);
                        job();
                    }
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
fn main() {
    let pool = ThreadPool::new(8);

    println!("Start pool with {} threads", pool.workers.len());

    //Execute some jobs
    for _ in 0..10 {
        pool.execute(|| {
            thread::sleep(Duration::from_millis(100));
            42
        });
    }

    // Shut down the thread pool
    pool.shutdown();

    println!("Thread pool shut down");
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use super::*;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
        pool.shutdown();
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_creation_with_zero_threads() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_single_job() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            42
        });

        std::thread::sleep(Duration::from_millis(100));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
        pool.shutdown();
    }

    #[test]
    fn test_execute_multiple_jobs() {
        let pool = ThreadPool::new(3);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                1
            });
        }

        thread::sleep(Duration::from_millis(500));

        assert_eq!(counter.load(Ordering::SeqCst), 10);
        pool.shutdown();
    }

    #[test]
    fn test_execute_batch_returns_correct_results() {
        let pool = ThreadPool::new(4);

        let jobs: Vec<WorkItem> = (1..=5)
            .map(|i| Box::new(move || i * 2) as WorkItem)
            .collect();

        let results = pool.execute_batch(jobs);

        assert_eq!(results, vec![2, 4, 6, 8, 10]);
        pool.shutdown();
    }

    #[test]
    fn test_execute_batch() {
        let pool = ThreadPool::new(4);
        let jobs: Vec<WorkItem> = (0..10)
            .map(|_| {
                let job = move || 42;
                Box::new(job) as WorkItem
            })
            .collect();

        let results = pool.execute_batch(jobs);
        assert_eq!(results.len(), 10);
        assert!(results.iter().all(|&x| x == 42));
    }

    #[test]
    fn test_shutdown() {
        let pool = ThreadPool::new(4);
        pool.shutdown();
    }

    #[test]
    fn test_execute_and_shutdown() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = std::sync::mpsc::channel();

        pool.execute(move || {
            tx.send(42).unwrap();
            42
        });

        pool.shutdown();

        assert_eq!(rx.recv().unwrap(), 42);
    }
}