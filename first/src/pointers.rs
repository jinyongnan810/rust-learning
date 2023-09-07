#![deny(clippy::all)]

use std::ops::Deref;

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
}
