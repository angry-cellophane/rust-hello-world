mod m1 {
    #[derive(Debug)]
    pub struct Rectangle {
        height: u32,
        width: u32,
        used: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            return Self {
                width,
                height,
                used: 0,
            };
        }

        pub fn area(&mut self) -> u32 {
            self.update_counter();
            return self.width * self.height;
        }

        fn update_counter(&mut self) {
            self.used = self.used + 1;
        }

        pub fn height(&self) -> u32 {
            return self.height;
        }

        pub fn width(&self) -> u32 {
            return self.width;
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            return self.width >= other.width && self.height >= other.height;
        }
    }
}

fn main() {
    let mut r1 = m1::Rectangle::new(16, 33);
    let mut r2 = m1::Rectangle::new(23, 15);
    let square = r1.area();
    println!("{:?} square {square}", r1);
    println!("r1 can hold r2 = {0}", r1.can_hold(&r2));
}