use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    value: String,
}

fn main() {
    let mut m = HashMap::new();
    let i1 = Item {
        value: String::from("1"),
    };
    let i2 = Item {
        value: String::from("2"),
    };
    m.insert(String::from("1"), &i1);
    m.insert(String::from("2"), &i2);
    let e = m.entry(String::from("3")).or_insert(&i1);

    println!("{:?}", &i1);
    println!("{:?}", &i2);
    println!("{:?}", m);
}
