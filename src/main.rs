fn main() {
    let x: f64 = 10.0;
    let y: f64 = 15.0;
    println!("Area: {}", area(x, y));
}

fn area(x: f64, y: f64) -> f64{
    x * y
}