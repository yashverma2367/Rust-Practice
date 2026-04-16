#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn print_list<T: std::fmt::Debug>(list: &List<T>) {
    match list {
        List::Element(value, next) => {
            println!("{:?}", value);
            print_list(next);
        }
        List::Nil => {}
    }
}

pub fn use_box() {
    let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    print_list(&list);
}

use std::rc::Rc;

pub fn use_rc() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    dbg!(a);
    dbg!(b);
}
