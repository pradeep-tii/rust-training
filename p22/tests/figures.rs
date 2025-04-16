use p22::figures:: {
    Circle, Rectangle, Triangle, perimeter_circle, perimeter_rectangle, perimeter_triangle,
};

#[test]
fn test_perimeter_circle() {
    let circle = Circle { radius: 5.0 };
    assert_eq!(perimeter_circle(circle), 31.41592653589793);
}

#[test]
fn test_perimeter_triangle() {
    let triangle = Triangle {
        a: 2.464,
        b: 2.0,
        c: 3.0,
    };
    assert_eq!(perimeter_triangle(triangle), 7.464);
}

#[test]
fn test_perimeter_rectangle() {
    let rectangle = Rectangle {
        length: 5.0,
        width: 10.0,
    };
    assert_eq!(perimeter_rectangle(rectangle), 30.0);
}
