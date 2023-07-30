#[derive(Debug)]
struct Item {
    value: String,
}

fn do_stuff(item: &Item) {
    let mut v = vec![item];
    v.push(&Item {
        value: String::from("2"),
    });
}

fn main() {
    let i = Item {
        value: String::from("1"),
    };
    do_stuff(&i);
    println!("{:?}", &i);

}
