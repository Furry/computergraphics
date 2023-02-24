use super::{ Point, Color};
use crate::{Drawable, canvas::Canvas};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [Point; 3],
    pub center: Point,
    pub theta: f32
}

impl Triangle {
    pub fn from_size(width: f32, height: f32, center: Point) -> Self {
        // Apply the center point to the triangle
        Self {
            points: [
                Point::new(-width / 2.0, height / 2.0),
                Point::new(width / 2.0, height / 2.0),
                Point::new(0.0, -height / 2.0),
            ],
            center,
            theta: 0.0,
        }
    }

    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            points: [p1, p2, p3],
            center: Point::new(0.0, 0.0),
            theta: 0.0,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, canvas: &mut Canvas, color: Color) {
        // Generate lines from each point to the next

        let lines = [
            (self.points[0], self.points[1]),
            (self.points[1], self.points[2]),
            (self.points[2], self.points[0]),
        ];

        // Draw each line
        for line in lines.iter() {
            // Use canvas.draw_line
            canvas.draw_line(line.0, line.1, color);
        }
    }

    fn get_points(&self) -> Vec<Point> {
        todo!()
    }

    fn set_rotation(&mut self, theta: f32) {
        todo!()
    }

    fn fill(&self, canvas: &mut Canvas, color: Color) {
        let (x1, x2, x3, y1, y2, y3) = (
            self.points[0].x,
            self.points[1].x,
            self.points[2].x,
            self.points[0].y,
            self.points[1].y,
            self.points[2].y,
        );

        let d1 = f32::sqrt((x2 - x1).powi(2) + (y2 - y1).powi(2));
        let d2 = f32::sqrt((x3 - x2).powi(2) + (y3 - y2).powi(2));
        let d3 = f32::sqrt((x1 - x3).powi(2) + (y1 - y3).powi(2));

        // Orientate the triangle so that the longest side is horizontal (flat on the bottom)
        let (x1, x2, x3, y1, y2, y3) = if d1 > d2 && d1 > d3 {
            (x1, x2, x3, y1, y2, y3)
        } else if d2 > d1 && d2 > d3 {
            (x2, x3, x1, y2, y3, y1)
        } else {
            (x3, x1, x2, y3, y1, y2)
        };

        let invslope1 = (x2 - x1) / (y2 - y1);
        let invslope2 = (x3 - x1) / (y3 - y1);

        let mut curx1 = x1;
        let mut curx2 = x1;

        for scanline_y in y1 as i32..y2 as i32 {
            let start = curx1 as i32;
            let end = curx2 as i32;

            for x in start..end {
                canvas.draw_line(
                    Point::new(x as f32, scanline_y as f32),
                    Point::new(x as f32, scanline_y as f32),
                    color,
                );
            }

            curx1 += invslope1;
            curx2 += invslope2;
        }

    }
}

impl Triangle {
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }

    pub fn fill_gradient(&self, canvas: &mut Canvas, color: Color) {
        let (x0, x1, x2, y0, y1, y2) = (
            self.points[0].x,
            self.points[1].x,
            self.points[2].x,
            self.points[0].y,
            self.points[1].y,
            self.points[2].y,
        );

        // Give each point a h value for color in rgb.
        let (h0, h1, h2) = (0.0, 0.5, 1.0);

        let mut x01 = Point::interpolate(y0, x0, y1, x1);
        let mut h01 = Point::interpolate(y0, h0, y1, h1);
        let x12 = Point::interpolate(y1, x1, y2, x2);
        let h12 = Point::interpolate(y1, h1, y2, h2);
        let x02 = Point::interpolate(y0, x0, y2, x2);
        let h02 = Point::interpolate(y0, h0, y2, h2);


        // // Concatinate the short sides
        x01.pop();
        let x012 = [x01, x12].concat();

        h01.pop();
        let h012 = [h01, h12].concat();

        let m = x012.len() / 2;
        let (x_left, h_left, x_right, h_right) = {
            if x02[m] < x012[m] {
                (x02, h02, x012, h012)
            } else {
                (x012, h012, x02, h02)
            }
        };

        for y in y0 as i32..y2 as i32 {
            let indx = (y - y0 as i32) as usize;
            let x_l = x_left.get(indx).unwrap().to_owned();
            let x_r = x_right.get(indx).unwrap().to_owned();

            let h_segment = Point::interpolate(x_l, h_left[indx], x_r, h_right[indx]);
            for x in x_l as i32..x_r as i32 {
                let shaded = color * h_segment[(x - x_l as i32) as usize];
                canvas.put_pixel(
                    x as f32,
                    y as f32,
                    shaded,
                );
            }
        }


    }
}
