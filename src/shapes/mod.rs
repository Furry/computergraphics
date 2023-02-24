pub mod rectangle;
pub mod triangle;
pub mod geometry;
pub mod colors;

// reexport rectangle
pub use rectangle::Rectangle;
pub use triangle::Triangle;

use crate::Canvas;

pub use self::geometry::Point;
pub use self::colors::Color;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, color: Color);
    fn get_points(&self) -> Vec<Point>;
    fn set_rotation(&mut self, theta: f32);
    fn fill(&self, canvas: &mut Canvas, color: Color);
}

pub trait Positionable {
    fn set_position(&mut self, x: f32, y: f32);
    fn get_position(&self) -> Point;
}