enum Shape {
    Rectangle(f64, f64),
    Circle(f64)
}

pub fn enums(){
    let rect = Shape::Rectangle(1.0, 2.0);
    println!("rectangle area: {}", print_area(rect));
    let circle = Shape::Circle(5.0);
    println!("circle area: {}", print_area(circle));
}

fn print_area(shape: Shape) -> f64{
    // pattern matching
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r
    };
    return area;
}