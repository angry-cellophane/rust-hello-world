#[derive(Debug)]
struct Item {
    value: String,
}

fn print_item(item: &Item) {
    println!("item = {:?}", item);
}

fn main() {
    let v = vec![
        Item {
            value: String::from("1"),
        },
        Item {
            value: String::from("2"),
        },
        Item {
            value: String::from("3"),
        },
    ];
    print_item(&v[0]);
    print_item(&v[1]);
    print_item(&v[0]);

    let mut v2 = v;
    v2.push(Item {
        value: String::from("4"),
    });
    v2.insert(0, Item {
        value: String::from("5"),
    });

    print_item(&v2[0]);
}
