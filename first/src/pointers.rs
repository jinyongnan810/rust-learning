#![deny(clippy::all)]

use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

struct BoxedValue<T> {
    value: T,
}
impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}
// allow dereferencing of BoxedValue(using *)
impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_number(number: &i32) {
    println!("number is {}", number);
}
struct Person {
    name: String,
    age: Cell<u8>,
}

pub fn pointers() {
    println!("----------pointers----------");
    let age = Box::new(37);
    println!("Age is {}", age);
    println!("double age is {}", *age * 2);

    let self_made_box = BoxedValue::new(37);
    // *ptr is equivalent to *(ptr.deref())
    println!("Age is {}", *self_made_box);
    println!("double age is {}", self_made_box.deref() * 2);
    let ref_of_self_made_box = self_made_box.deref();
    println!("Age is {}", *ref_of_self_made_box);
    // passing a reference of box to a function
    print_number(&self_made_box);

    // reference counting
    let array = [1, 2, 3];
    let rc = Rc::new(array);
    println!("array = {:?}", array);
    println!("rc = {:?}", rc);
    drop(rc);
    // println!("rc = {:?}", rc)
    println!("array = {:?}", array);

    // Cell allows mutation inside immutable references
    let person = Person {
        name: String::from("Alice"),
        age: Cell::new(37),
    };
    person.age.set(38);
    println!("Person({})'s age is now {}", person.name, person.age.get());
}
