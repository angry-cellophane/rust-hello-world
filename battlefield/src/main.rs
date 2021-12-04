fn main() {
    let x = 6;
    println!("1: x = {}", x);

    let x = 7;
    println!("2: x = {}", x);
    {
        let x = 8;
        println!("3: x = {}", x);
    }
    println!("4: x = {}", x);
}
