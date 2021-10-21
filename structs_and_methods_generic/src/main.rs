use structs_and_methods::{Point, Figure, Rect, Circle};

fn main() {
    let left_up = Point { x: 0.0, y: 4.0 };
    let right_down = Point { x: 6.0, y: 0.0 };
    let rect_figure = Figure::Rect(Rect { left_up, right_down });
    let circle_figure = Figure::Circle(Circle { center: left_up, r: 3.0 });
    println!("Rectangle area: {}", rect_figure.area());
    println!("Circle area: {}", circle_figure.area());
    let circle_point = Point { x: -2.0, y: 4.0 };
    let rect_point = Point { x: 5.0, y: 1.0 };
    let common_point = Point { x: 1.0, y: 3.0 };
    println!("Point ({}, {}) in circle: {}, but in rectangle: {}", circle_point.x, circle_point.y, circle_figure.contains(&circle_point), rect_figure.contains(&circle_point));
    println!("Point ({}, {}) in circle: {}, but in rectangle: {}", rect_point.x, rect_point.y, circle_figure.contains(&rect_point), rect_figure.contains(&rect_point));
    println!("Point ({}, {}) in circle: {}, but in rectangle: {}", common_point.x, common_point.y, circle_figure.contains(&common_point), rect_figure.contains(&common_point));
}
