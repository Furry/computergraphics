use crate::Canvas;
use crate::shapes::Drawable;

use super::triangle::Triangle;
use super::{ Color, Point };

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub triangles: [Triangle; 2],
    pub center: Point,
    pub theta: f32
}

impl Rectangle {
    pub fn new(width: f32, height: f32, center: Point) -> Self {

        // Get all four points of the rectangle
        let p1 = Point::new(-width / 2.0, height / 2.0);
        let p2 = Point::new(width / 2.0, height / 2.0);
        let p3 = Point::new(width / 2.0, -height / 2.0);
        let p4 = Point::new(-width / 2.0, -height / 2.0);

        Self {
            triangles: [
                Triangle::new(p1, p2, p3),
                Triangle::new(p1, p3, p4),
            ],
            center,
            theta: 0.0,
        }
    }
}

impl Drawable for Rectangle {
    fn set_rotation(&mut self, theta: f32) {
        todo!()
    }

    fn draw(&self, canvas: &mut Canvas, color: Color) {
        for triangle in self.triangles.iter() {
            triangle.draw(canvas, color);
        }
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for triangle in self.triangles.iter() {
            points.append(&mut triangle.get_points());
        }

        points
    }

    fn fill(&self, canvas: &mut Canvas, color: Color) {
        for triangle in self.triangles.iter() {
            triangle.fill(canvas, color);
        }
    }
}