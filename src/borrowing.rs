#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

pub fn borrow() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2); // borrows the pointers so doesnt modify their values (p1, p2)
    println!("{p1:?} + {p2:?} = {p3:?}");
}

pub fn borrow_error() {
    let mut vec = vec![1, 2, 3, 4, 5];
    // let elem = &mut vec[2];
    vec.push(6);
    // dbg!(elem);
}

pub fn borrow_error_2() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let clone = vec.clone();
    for elem in clone {
        vec.push(elem * 2);
    }
    // for elem in &vec {
    //     vec.push(elem * 2);
    // }
}
