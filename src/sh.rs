#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin(); //stack allocation
    let p2 = Box::new(origin()); //heap allocation

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); //actual data is stored on the stack, 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); //pointer size is 8 bytes on x64 systems

    let p3 = *p2; //every boxed or heap object has a pointer and with * we can create another new pointer for that obj on the stack
    println!("{}",p3.x);
}
