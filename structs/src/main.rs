#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing methods
impl Rectangle {
    fn area(&self) -> u32 {
        // borrow self. If you want to modify self, use &mut self
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Method call
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    // Associated function call
    let sq = Rectangle::square(3); // no need to use Rectangle::square(&3)
    println!("The area of the square is {} square pixels.", sq.area());
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
