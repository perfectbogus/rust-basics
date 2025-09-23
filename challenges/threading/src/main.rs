use std::thread;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

struct ThreadedCounter {
    counter: Arc<Mutex<i32>>
}

impl ThreadedCounter {

    fn new(initial_value: i32) -> Self {
        ThreadedCounter {
            counter: Arc::new(Mutex::new(initial_value))
        }
    }

    fn spawn_incrementer(&self, increments: i32, delay_ms: u64) -> thread::JoinHandle<()> {
        let counter_clone = Arc::clone(&self.counter);
        thread::spawn(move || {
            for i in 0..increments {
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                    println!("Incremented to: {}", *num);
                }

                if i < increments - 1 {
                    sleep(Duration::from_millis(delay_ms));
                }
            }
        })
    }

    fn spawn_decrementer(&self, decrements: i32, delay_ms: u64) -> thread::JoinHandle<()> {
        let counter_clone = Arc::clone(&self.counter);
        thread::spawn(move || {
            for i in 0..decrements {
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num -= 1;
                    println!("Decremented to: {}", *num);
                }

                if i < decrements - 1 {
                    sleep(Duration::from_millis(delay_ms));
                }
            }
        })
    }

    fn get_value(&self) -> i32 {
        *self.counter.lock().unwrap()
    }

    fn reset(&self) {
        let mut num = self.counter.lock().unwrap();
        *num = 0;
    }
}

fn main() {
    let counter = ThreadedCounter::new(0);
    println!("Initial Value: {}", counter.get_value());

    let inc_handle = counter.spawn_incrementer(5, 100);
    let dec_handle = counter.spawn_decrementer(3, 150);

    inc_handle.join().unwrap();
    dec_handle.join().unwrap();

    println!("Final Value: {}", counter.get_value());

}
