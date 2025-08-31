use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::RefCell;
fn main() {
    reference_counted();

    atomic_reference_counted();

    interior_mutability();

    reference_counted_with_mutability();

    reference_thread_safe_shared_mutation();

    let counter = Rc::new(RefCell::new(0));

    let c1 = counter.clone();
    let c2 = counter.clone();

    {
        let mut increment = c1.borrow_mut();
        *increment += 1;
    }

    // Read through c2
    println!("Counter value: {:?}", c2.borrow());

    // Should print: Counter value: 1
}



fn reference_thread_safe_shared_mutation() {
    println!("### Running reference thread safe shared mutation ###");
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data_clone = shared_data.clone();

    let handle = thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data.push(4);
    });

    handle.join().unwrap();
    println!("data: {:?}", shared_data.lock().unwrap());
}


fn reference_counted_with_mutability() {
    println!("### Running reference counted with mutability ###");
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    println!("Before mutate SharedData: {:?}", shared_data.borrow());
    let owner1 = shared_data.clone();
    let owner2 = shared_data.clone();

    owner1.borrow_mut().push(4);

    // Read through owner2
    println!("From owner2: {:?}", owner2.borrow());

    // Read from original
    println!("From Original: {:?}", shared_data.borrow());
}

fn interior_mutability() {
    println!("### interior mutability ###");

    let data = RefCell::new(vec![1, 2, 3]);
    println!("Before to mutate: {:?}", data);

    {
        let mut borrowed = data.borrow_mut();
        borrowed.push(4);
    }

    let borrowed = data.borrow();
    println!("Data: {:?}", *borrowed);
}

// Arc - Atomic Reference Counted
fn atomic_reference_counted() {
    println!("### atomic reference counted ###");
    let data = Arc::new(vec![1, 2, 3, 4]);

    let data1 = data.clone();
    let data2 = data.clone();

    let handle1 = thread::spawn(move || {
        println!("data1: {:?}", data1);
        let sum = data1.iter().sum::<i32>();
        println!("data1 sum: {}", sum);
    });

    let handle2 = thread::spawn(move || {
        println!("data2: {:?}", data2);
        let mul = data2.iter().product::<i32>();
        println!("data2 mul: {}", mul);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// Rc - Reference Counted
fn reference_counted() {
    println!("### reference counted ###");
    let data = Rc::new(String::from("Shared data"));

    let owner1 = data.clone();
    let owner2 = data.clone();

    println!("Owner 1: {owner1}");
    println!("Owner 2: {owner2}");
    println!("Data: {data}");
}