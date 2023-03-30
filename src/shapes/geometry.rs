use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Point3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.y.partial_cmp(&other.y).unwrap())
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Set of points between two points.
    pub fn interpolate(i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
        let mut points = Vec::new();

        let a = (d1 - d0) / (i1 - i0);
        let d = d0;

        for i in i0 as usize..i1 as usize {
            points.push(a * (i as f32) + d);
        }

        points
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl Into<sdl2::rect::Point> for Point {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }
}

