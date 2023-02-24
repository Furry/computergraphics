#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Impl mul for color
impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * rhs) as u8,
            g: (self.g as f32 * rhs) as u8,
            b: (self.b as f32 * rhs) as u8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gradient {
    pub steps: Vec<Color>,
    index: u32,
    stop: u32
}

impl Gradient {
    pub fn new(steps: Vec<Color>, end: u32) -> Self {
        Self {
            steps,
            index: 0,
            stop: end
        }
    }

    pub fn get_color(&self, index: usize) -> Color {
        self.steps[index]
    }

    // pub fn next(&mut self) -> Color {
    //     // Get the current color by splitting the index by the number of steps.
    //     let color = self.get_color((self.index / self.stop) as usize);
    // }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.r, self.g, self.b)
    }
}
