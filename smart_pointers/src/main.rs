use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("Hello, world! {b}");

    header("Following the Pointer to the Value");
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("X: {}, Y: {}", x, *y);

    header("Using Box<t> like a reference");
    let x2 = 5;
    let y2 = Box::new(x2);
    println!("X: {}, Y: {}", x2, *y2);

    header("Defining our own Smart Pointer");
    let x3 = 5;
    let y3 = MyBox(x3);
    assert_eq!(5, x3);
    assert_eq!(5, *y3);
    println!("X: {}, Y: {}", x3, *y3);

    header("Implicit defer coercions with functions and methods");
    hello("Rust");
    let m = MyBox(String::from("Rust"));
    hello(&m);

    header("having multiple owners of mutable data by combining Rc<T> and RefCell<T>");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn header(message: &str) {
    println!();
    println!("#############################");
    println!("### {message} ###");
    println!("#############################")
}