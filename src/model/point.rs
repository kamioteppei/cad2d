#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[allow(dead_code)]
    pub fn base() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new( x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}