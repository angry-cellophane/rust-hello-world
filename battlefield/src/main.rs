fn main() {
    let (x,y,z) = create_tuple();
    println!("{} {} {}", x, y, z);
}

fn create_tuple() -> (i32, f64, u8) {
    return (500, 6.4, 1);
}
