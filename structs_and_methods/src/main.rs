#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone)]
struct Rect {
    left_up: Point,
    right_down: Point,
}

#[derive(Copy, Clone)]
struct Circle {
    center: Point,
    r: f64,
}

#[derive(Copy, Clone)]
enum Figure {
    Rect(Rect),
    Circle(Circle),
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        return p.x >= self.left_up.x && p.x <= self.right_down.x &&
            p.y <= self.left_up.y && p.y >= self.right_down.y;
    }
    fn area(&self) -> f64 {
        return (self.right_down.x - self.left_up.x) * (self.left_up.y - self.right_down.y);
    }
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        return (self.center.x - p.x).hypot(self.center.y - p.y) <= self.r;
    }
    fn area(&self) -> f64 {
        return 2.0 * std::f64::consts::PI * self.r;
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Figure::Rect(rect) => { rect.contains(p) }
            Figure::Circle(circle) => { circle.contains(p) }
        }
    }
    fn area(&self) -> f64 {
        match self {
            Figure::Rect(rect) => { rect.area() }
            Figure::Circle(circle) => { circle.area() }
        }
    }
}

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
