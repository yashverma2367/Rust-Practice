use std::rc::Rc;
#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}
struct Cat {
    lives: i8,
}
struct Dog {
    name: String,
    age: i8,
}
trait Pet {
    fn talk(&self) -> String;
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

pub fn use_rc() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    dbg!(a);
    dbg!(b);
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

pub fn trait_in_box() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Fido"),
            age: 5,
        }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
}
