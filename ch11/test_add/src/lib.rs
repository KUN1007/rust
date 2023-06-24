#[derive(Debug)]
pub struct Rectangle {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hold() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };

        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn azkhx() {
        assert_eq!(2 * 2, 4);
    }
}
