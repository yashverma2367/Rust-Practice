fn say_hello(name: String) {
    println!("Hello {name}")
}

pub fn tranfer_ownership() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
}

pub fn out_of_scope() {
    {
        let n = String::from("Yash");
    }
    // println!("{n}"); //NOT GONNA WORK BECAUSE OUT OF SCOPE
}

pub fn move_ownership() {
    let s1 = String::from("Not Yash");
    let s2 = s1; //s1 transfered ownership to s2 and cannot be used anymore
    // println!("{s1}"); //Wont Work,/
    println!("{s2}") //Will work has ownership
}

pub fn clone_value() {
    let s1 = String::from("Not Yash");
    let s2 = s1.clone(); //s2 cloned s1 value now has a value independent of s1;
    println!("{s1}"); //Will Work,
    println!("{s2}") //Will Work
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

pub fn copy_types() {
    let p1 = Point(3, 4);
    let p2 = p1; //Also copies the types over
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}