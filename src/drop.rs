struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

pub fn use_drop() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting innermost block c: {} d: {}", c.name, d.name);
        }
        println!("Exiting next block b: {}", b.name);
    }
    drop(a);
    println!("Exiting main");
}