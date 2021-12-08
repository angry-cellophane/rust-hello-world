fn main() {
    let tup = (500, 6.4, 1);

    println!("tup {:?}", create_tuple());
    println!("Tup1 {}", tup.0);
    println!("Tup2 {}", tup.1);
    println!("Tup3 {}", tup.2);
    println!("The value of tup is: {:?}", tup);
}

fn create_tuple() -> (i32, f64, u8) {
    return (500, 6.4, 1);
}
