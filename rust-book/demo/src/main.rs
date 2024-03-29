#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let s = Rectangle::square(20);

    let rect1: Rectangle = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 20,
        length: 30,
    };

    // println!("{}", rect.area());

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect2.can_hold(&rect3));
    println!("{:#?}", rect1)
}
