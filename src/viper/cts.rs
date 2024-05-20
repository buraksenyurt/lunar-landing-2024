use sdl2::rect::Point;

#[derive(Clone)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}
impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Default, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

pub struct Location {
    pub x: i32,
    pub y: i32,
}
#[allow(dead_code)]
impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Rectangle {
    pub location: Location,
    pub dimension: Dimension,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(location: Location, dimension: Dimension) -> Self {
        Self {
            location,
            dimension,
        }
    }
}
