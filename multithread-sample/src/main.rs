use std::{time, thread};

fn main() {
    let n1 = 1200;
    let n2 = 1000;

    let child = thread::spawn(move || {
        heavy_calc("child", n2)
    });

    let s1 = heavy_calc("main", n1);

    match child.join() {
        Ok(s2) => println!("{}, {}", s1, s2),
        Err(e) => println!("error: {:?}", e),
    }
}

fn heavy_calc(name: &str, n: u64) -> u64 {
    println!("{}: started.", name);

    thread::sleep(time::Duration::from_millis(n));
    let sum = (1..n).sum();
    println!("{}: ended.", name);
    sum
}
