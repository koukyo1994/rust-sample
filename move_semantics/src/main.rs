use std::ops::Drop;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

fn main() {
    let mut p1 = Parent(2, Child(11), Child(12));
    let p2 = p1;

    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); => コメントを外すとエラー

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1);
}
