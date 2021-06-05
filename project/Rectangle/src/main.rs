fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.show_width();
    Rectangle::static_fun();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show_width(&self) {
        println!("width = {}", self.width);
    }

    fn static_fun() {
        println!("hello world");
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}