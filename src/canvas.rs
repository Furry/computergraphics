
use crate::{Color};
use crate::shapes::geometry::Point;

pub struct Canvas {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl Canvas {
    pub fn put_pixel(&mut self, x: f32, y: f32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(Point::new(x, y)).unwrap();
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let (x1, x2) = (start.x as i32, end.x as i32);
        let (y1, y2) = (start.y as i32, end.y as i32);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.put_pixel(x as f32, y as f32, color);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy as i32 {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::new(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}