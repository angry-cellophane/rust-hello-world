use crate::messages::Message;

mod ip {
    #[derive(Debug)]
    pub enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

}

mod messages {
    use std::process::exit;

    pub enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        pub fn call(&self) {
            match &self {
                Message::Quit => {
                    println!("byee!");
                },
                Message::Write(message) => {
                    println!("writing message {message}");
                },
                Message::Move {x, y} => {
                    println!("moving to ({x},{y})");
                },
                Message::ChangeColor(r,g,b) => {
                    println!("changing colour to ({r},{g},{b})");
                }
            }
        }
    }
}

fn main() {
    let home = ip::IpAddrKind::V4(127, 0, 0, 1);
    let loopback = ip::IpAddrKind::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);

    Message::Quit.call();
    Message::Write(String::from("hello")).call();

    let x = Some(5);
    let y: Option<u32> = None;
    let s = x.or(Some(0)).unwrap() + y.or(Some(0)).unwrap();
    println!("sum = {s}");
    if let Some(x) = x {
        println!("x = {x}");
    }
}
