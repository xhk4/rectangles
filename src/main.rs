struct Rect {
    width: f64,
    height: f64
}

fn main() {
    let rect = Rect {
        width: 10.0,
        height: 15.5,
    };

    println!("Area: {}", area(&rect));
}

fn area(rect: &Rect) -> f64 {
    rect.width * rect.height
}