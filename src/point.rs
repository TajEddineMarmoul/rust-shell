use std::write;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
    pub fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: ({},{})", self.x, self.y)
    }
}
