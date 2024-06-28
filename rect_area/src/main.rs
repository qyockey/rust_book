#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height, }
    }


    fn copy(other: &Rectangle) -> Self {
        Self { ..*other }
    }


    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }


    fn area(&self) ->u32 {
        self.width * self.height
    }


    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::copy(&rect2);
    let square = Rectangle::square(35);

    println!("Rectangle: {rect1:?}");
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}


