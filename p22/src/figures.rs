
pub struct Circle {
    pub radius: f64,
}

pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub struct Rectangle{
    pub length: f64,
    pub width: f64,
}

// enum Shape {
//     Circle(Circle),
//     Triangle(Triangle),
//     Rectangle(Rectangle),
// }

pub fn area_circle(circle: Circle) -> f64 {
    std::f64::consts::PI * circle.radius * circle.radius
    }

pub fn area_triangle(triangle: Triangle) -> f64 {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

pub fn area_rectangle(rectangle: Rectangle) -> f64 {
    rectangle.length * rectangle.width
}

pub fn perimeter_circle(circle: Circle) -> f64 {
    2.0 * std::f64::consts::PI * circle.radius
}

pub fn perimeter_triangle(triangle: Triangle) -> f64 {
    let a = triangle.a;
    let b = triangle.b;
    let c = triangle.c;

    a + b + c
}
pub fn perimeter_rectangle(rectangle: Rectangle) -> f64 {
    2.0 * (rectangle.length + rectangle.width)
}


#[test]
fn test_area_circle() {
    let circle = Circle {
        radius: 1.0,
    };
    assert_eq!(area_circle(circle), std::f64::consts::PI);
}

#[test]
fn test_area_triangle() {
    let triangle = Triangle{a: 3.0, b: 4.0, c: 5.0};
    
    assert_eq!(area_triangle(triangle), 6.0);
}

#[test]
fn test_area_rectangle() {
    let rectangle = Rectangle {length: 3.0, width: 4.0,};
    assert_eq!(area_rectangle(rectangle), 12.0);
}

