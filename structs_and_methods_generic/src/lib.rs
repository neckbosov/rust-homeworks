use noisy_float::types::r64;
use num_traits::Num;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point<T: Num + Debug + Clone + Copy + PartialOrd> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect<T: Num + Debug + Clone + Copy + PartialOrd> {
    pub left_up: Point<T>,
    pub right_down: Point<T>,
}

#[derive(Copy, Clone, Debug)]
pub struct Circle<T: Num + Debug + Clone + Copy + PartialOrd> {
    pub center: Point<T>,
    pub r: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Figure<T: Num + Debug + Clone + Copy + PartialOrd> {
    Rect(Rect<T>),
    Circle(Circle<T>),
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Rect<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        return p.x >= self.left_up.x
            && p.x <= self.right_down.x
            && p.y <= self.left_up.y
            && p.y >= self.right_down.y;
    }
    pub fn area(&self) -> f64
    where
        T: Into<f64>,
    {
        (self.right_down.x.into() - self.left_up.x.into())
            * (self.left_up.y.into() - self.right_down.y.into())
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Circle<T> {
    pub fn contains(&self, p: &Point<T>) -> bool
    where
        T: Into<f64>,
    {
        (self.center.x - p.x)
            .into()
            .hypot((self.center.y - p.y).into())
            <= self.r
    }
    pub fn area(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Figure<T> {
    pub fn contains(&self, p: &Point<T>) -> bool
    where
        T: Into<f64>,
    {
        match self {
            Figure::Rect(rect) => rect.contains(p),
            Figure::Circle(circle) => circle.contains(p),
        }
    }
    pub fn area(&self) -> f64
    where
        T: Into<f64>,
    {
        match self {
            Figure::Rect(rect) => rect.area(),
            Figure::Circle(circle) => circle.area(),
        }
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Hash> Hash for Point<T> {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        self.x.hash(&mut state);
        self.y.hash(&mut state);
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Hash> Hash for Rect<T> {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        self.left_up.hash(&mut state);
        self.right_down.hash(&mut state);
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> PartialEq for Circle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && r64(self.r) == r64(other.r)
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Hash> Hash for Circle<T> {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        self.center.hash(&mut state);
        r64(self.r).hash(&mut state);
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Eq> Eq for Point<T> {}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Eq> Eq for Rect<T> {}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Eq> Eq for Circle<T> {}

impl<T: Num + Debug + Clone + Copy + PartialOrd + Hash> Hash for Figure<T> {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        match self {
            Figure::Rect(rect) => rect.hash(&mut state),
            Figure::Circle(circle) => circle.hash(&mut state),
        }
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Eq for Figure<T> {}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Default for Point<T> {
    fn default() -> Self {
        Point {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Default for Rect<T> {
    fn default() -> Self {
        Rect {
            left_up: Point {
                x: T::zero(),
                y: T::one(),
            },
            right_down: Point {
                x: T::one(),
                y: T::zero(),
            },
        }
    }
}

impl<T: Num + Debug + Clone + Copy + PartialOrd> Default for Circle<T> {
    fn default() -> Self {
        Circle {
            center: Point::default(),
            r: 1.0,
        }
    }
}
