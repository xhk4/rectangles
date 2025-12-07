struct Rect {
    width: f64,
    height: f64
}

impl Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rect {
    fn square(size: f64) -> Rect {
        Rect {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 10.0,
        height: 15.5,
    };
    let rect2 = Rect::square(9.0);
    let rect3 = Rect::square(12.0);

    println!("rect1 Area: {}", rect1.area());
    println!("rect2 Area: {}, can hold: {}", rect2.area(), rect1.can_hold(rect2));
    println!("rect3 Area: {}, can hold: {}", rect3.area(), rect1.can_hold(rect3));
}
