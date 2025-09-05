use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::{BorrowError, Ref, RefCell};

fn main() {
    reference_counted();

    atomic_reference_counted();

    interior_mutability();

    reference_counted_with_mutability();

    reference_thread_safe_shared_mutation();

    simple_shared_ownership();

    challenge_one_basic_immutable_sharing();

    println!("### Challenge two interior mutability ###");
    challenge_two_interior_mutability();

    println!("### Challenge three Shared Mutable State ###");
    challenge_three_shared_mutable_state();

    println!("### Challenge four: Error Handling with RefCell ###");
    challenge_four_error_handling_refcell();

    println!("### Challenge five: weak references ###");
    challenge_five_weak_references();

}

struct Parent {
    children: RefCell<Vec<Rc<Child>>>,
}

struct Child {
    parent: Weak<Parent>,
}

impl Parent {
    fn new() -> Rc<Self> {
        Rc::new(Parent {
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(self: &Rc<Self>) -> Rc<Child> {
        let child = Rc::new(Child {
            parent: Rc::downgrade(self)
        });

        self.children.borrow_mut().push(child.clone());
        child
    }
}

impl Child {
    fn get_parent(&self) -> Option<Rc<Parent>> {
        self.parent.upgrade()
    }
}

fn challenge_five_weak_references() {

}


fn challenge_four_error_handling_refcell() {
    let data = RefCell::new(vec![1, 2, 3]);

    // this should work fine
    {
        let borrowed = data.borrow();
        println!("Data: {:?}", borrowed);
    }

    let _immutable_ref = data.borrow();

    match data.try_borrow_mut() {
        Ok(mut data) => {
            data.push(4);
            println!("Successfully modified: {:?}", *data);
        }
        Err(_) => {
            println!("Could not get mutable borrow - already borrowed immutably!");
        }
    }

}

fn challenge_three_shared_mutable_state() {
    // Create a shared shopping cart (Vec<String>)
    let cart = Rc::new(RefCell::new(Vec::<String>::new()));

    let shopper1 = cart.clone();
    let shopper2 = cart.clone();
    let shopper3 = cart.clone();

    {
        shopper1.borrow_mut().push(String::from("apples"));
    }

    {
        shopper2.borrow_mut().push(String::from("bananas"));
    }

    {
        shopper3.borrow_mut().push(String::from("oranges"));
    }

    println!("Final Cart: {:?}", cart);

    // Bonus: Create a function that takes any shopper and adds an item
    add_item(shopper1, "milk".to_string());

    println!("After adding milk: {:?}", cart);
}

fn add_item(shopper: Rc<RefCell<Vec<String>>>, item: String) {
    shopper.borrow_mut().push(item);
}


struct BankAccount {
    balance: RefCell<i32>,
}

impl BankAccount {
    fn new(initial_balance: i32) -> Self {
        BankAccount { balance: RefCell::new(initial_balance)}
    }

    fn deposit(&self, amount: i32) {
        *self.balance.borrow_mut() += amount;
    }

    fn withdraw(&self, amount: i32) -> bool {
        if amount < *self.balance.borrow() {
            *self.balance.borrow_mut() -= amount;
            true
        } else {
            false
        }
    }

    fn balance(&self) -> i32 {
        *self.balance.borrow()
    }
}

fn challenge_two_interior_mutability() {
    let account = BankAccount::new(100);

    account.deposit(50);
    println!("{:?}", account.balance());

    let success = account.withdraw(30);
    println!("Withdrawal success: {}, Balance: {}", success, account.balance());

    let failed = account.withdraw(200);
    println!("Withdrawal failed with error: {}, Balance: {}", failed, account.balance());
}

fn challenge_one_basic_immutable_sharing() {
    let book = Rc::new(String::from("The book"));

    let reader1 = book.clone();
    let reader2 = book.clone();
    let reader3 = book.clone();

    println!("reader1: {:?}", reader1);
    println!("reader2: {:?}", reader2);
    println!("reader3: {:?}", reader3);

    println!("Reference Count: {}", Rc::strong_count(&reader1));
}

fn simple_shared_ownership() {
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