mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s)
}

fn main() {
    let items = vec![String::from("Hi")];
    let mut basket1 = Basket::new(String::from("Hi"));
    let mut stack1 = Stack::new(items);
    add_string(&mut basket1, String::from("Hi"));
    add_string(&mut stack1, String::from("Hi"));
    println!("{:#?}", basket1);
    println!("{:#?}", stack1);
}
