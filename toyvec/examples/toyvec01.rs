use toyvec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push(String::from("Java Finch"));
    v.push(String::from("Budgerigar"));

    let e = v.get(1);

    assert_eq!(e, Some(&"Budgerigar".to_string()));
}
