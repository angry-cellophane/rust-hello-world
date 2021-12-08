use std::io::stdin;

fn main() {
    let a = [1, 2, 3];

    let mut index = String::new();
    stdin().read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse()
        .expect("not a number");

    let element = a[index];
    println!("{}", element);
}