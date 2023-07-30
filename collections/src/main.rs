#[derive(Debug)]
struct Item {
    value: String,
}

fn main() {
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 1;
    }
    for i in &v {
        println!("el = {}", i);
    }
}
