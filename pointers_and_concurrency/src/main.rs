use std::{ops::Deref, rc::Rc};

mod concurrency;

use self::List::{Cons, Nil};
use self::List2::{Cons2, Nil2};
fn main() {
    println!("Hello, world!");

    // Box<T> for storing data on the heap --> for ex. recursive data types
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{list:?}");

    let x = 6;
    let y = &x;

    println!("{x}, {}", *y);

    // alternate way; using Box<T> like a ref
    let y1 = Box::new(x);
    assert_eq!(6, *y1);

    // with custom smart pointer
    let y2 = MyBox::new(x);
    assert_eq!(6, *y2);

    // Drop trait
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };

    drop(c);

    let d = CustomSmartPointer {
        data: String::from("world"),
    };

    println!("Blah");

    // Rc instead of Box for multiple ownership with clone
    let a1 = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Current count a1: {}", Rc::strong_count(&a1));
    let b1 = Cons2(3, Rc::clone(&a1));
    println!("Current count b1: {}", Rc::strong_count(&a1));
    let c1 = Cons2(4, Rc::clone(&a1)); // Not allowed as value moved above
    println!("Current count c1: {}", Rc::strong_count(&a1));

    // Concurrency
    concurrency::run();
}

#[derive(Debug, Clone)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug, Clone)]
enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

// Custom Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implement DeRef trait; enables a type to be treated like a reference
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Drop trait to cleanup resources when they go out of scope
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer: {}", self.data);
    }
}

// Rc (Reference counted smart pointer) - only for single-threaded scenarios
